import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import type {
  Variable,
  GetVariablesRequest,
  GetVariablesResponse,
  DeleteVariableRequest,
  DeleteVariableResponse,
  ImportEnvRequest,
  ImportEnvResponse
} from '../types/database';
import { VariableModal } from './VariableModal';
import { DeleteConfirmModal } from './DeleteConfirmModal';
import { ExportModal } from './ExportModal';
import { useToast } from './Toast';
import { useKeyboardShortcuts } from '../hooks/useKeyboardShortcuts';
import { useDebounce } from '../hooks/useDebounce';
import { 
  Plus, Upload, Download, Search, Eye, EyeOff, Copy, 
  FilePenLine, Trash2, AlertTriangle, Inbox, X, CheckCircle2, XCircle,
  CheckSquare, Square, Trash, FileDown 
} from 'lucide-react';
import { VariableTableSkeleton } from './Skeleton';
import { validateVariable } from '../utils/variableValidation';
import { exportToEnv, exportToCSV, generateExportFilename, type ExportFormat } from '../utils/exportImport';
import './VariableList.css';

interface VariableListProps {
  environmentId: number;
  environmentName: string;
}

export const VariableList: React.FC<VariableListProps> = ({ environmentId, environmentName }) => {
  const [variables, setVariables] = useState<Variable[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');
  const [showCreateVar, setShowCreateVar] = useState(false);
  const [editingVar, setEditingVar] = useState<Variable | null>(null);
  const [deletingVar, setDeletingVar] = useState<Variable | null>(null);
  const [showValues, setShowValues] = useState<Record<number, boolean>>({});
  const [searchQuery, setSearchQuery] = useState('');
  const [showExportModal, setShowExportModal] = useState(false);
  const [selectedVariables, setSelectedVariables] = useState<Set<number>>(new Set());
  const [bulkMode, setBulkMode] = useState(false);
  const toast = useToast();

  // Debounce search query for better performance
  const debouncedSearchQuery = useDebounce(searchQuery, 300);

  // Memoize filtered variables to avoid unnecessary re-computation
  const filteredVariables = useMemo(() => {
    return variables.filter(variable =>
      variable.key.toLowerCase().includes(debouncedSearchQuery.toLowerCase()) ||
      (showValues[variable.id!] && variable.value.toLowerCase().includes(debouncedSearchQuery.toLowerCase()))
    );
  }, [variables, debouncedSearchQuery, showValues]);

  const loadVariables = useCallback(async () => {
    try {
      setIsLoading(true);
      setError('');
      const request: GetVariablesRequest = { environment_id: environmentId };
      const response = await invoke<GetVariablesResponse>('get_variables', { request });
      if (response.success) {
        setVariables(response.variables);
      } else {
        setError(response.message);
      }
    } catch (err) {
      console.error('Failed to load variables:', err);
      setError(err instanceof Error ? err.message : 'Failed to load variables');
    } finally {
      setIsLoading(false);
    }
  }, [environmentId]);

  useEffect(() => {
    loadVariables();
  }, [loadVariables]);

  const handleDeleteClick = useCallback((variable: Variable, event: React.MouseEvent) => {
    if (event.shiftKey) {
      // Shift tuşuna basılıysa direkt sil
      deleteVariable(variable);
    } else {
      // Normal durumda modal aç
      setDeletingVar(variable);
    }
  }, []);

  const deleteVariable = useCallback(async (variable: Variable) => {
    try {
        const request: DeleteVariableRequest = { id: variable.id! };
        const response = await invoke<DeleteVariableResponse>('delete_variable', { request });
        if (response.success) {
            toast.success(`Variable "${variable.key}" deleted`);
            await loadVariables();
        } else {
            throw new Error(response.message);
        }
    } catch (err) {
        toast.error(err instanceof Error ? err.message : 'Failed to delete variable');
    }
  }, [toast, loadVariables]);

  const confirmDelete = useCallback(async () => {
    if (!deletingVar) return;
    await deleteVariable(deletingVar);
  }, [deletingVar, deleteVariable]);

  const toggleShowValue = useCallback((varId: number) => {
    setShowValues(prev => ({ ...prev, [varId]: !prev[varId] }));
  }, []);

  const copyToClipboard = useCallback(async (text: string, customMessage?: string) => {
    try {
      await navigator.clipboard.writeText(text);
      toast.success(customMessage || 'Copied to clipboard!');
    } catch (err) {
      toast.error('Failed to copy');
    }
  }, [toast]);

  // Bulk operations
  const toggleSelectVariable = (varId: number) => {
    setSelectedVariables(prev => {
      const newSet = new Set(prev);
      if (newSet.has(varId)) {
        newSet.delete(varId);
      } else {
        newSet.add(varId);
      }
      return newSet;
    });
  };

  const selectAllVariables = () => {
    const allIds = new Set(filteredVariables.map(v => v.id!));
    setSelectedVariables(allIds);
  };

  const deselectAllVariables = () => {
    setSelectedVariables(new Set());
  };

  const handleBulkDelete = async () => {
    if (selectedVariables.size === 0) return;
    
    try {
      const deletePromises = Array.from(selectedVariables).map(async (varId) => {
        const variable = variables.find(v => v.id === varId);
        if (!variable) return;
        
        const request: DeleteVariableRequest = { id: varId };
        await invoke<DeleteVariableResponse>('delete_variable', { request });
      });

      await Promise.all(deletePromises);
      toast.success(`Deleted ${selectedVariables.size} variables`);
      setSelectedVariables(new Set());
      setBulkMode(false);
      await loadVariables();
    } catch (err) {
      toast.error('Failed to delete some variables');
      console.error('Bulk delete error:', err);
    }
  };

  const handleBulkExport = () => {
    if (selectedVariables.size === 0) return;
    setShowExportModal(true);
  };

  const handleExport = async (format: ExportFormat, options: { includeComments?: boolean; sortKeys?: boolean }) => {
    try {
      // Use selected variables if in bulk mode, otherwise all variables
      const varsToExport = selectedVariables.size > 0 
        ? variables.filter(v => selectedVariables.has(v.id!))
        : variables;
      
      let content: string;
      let defaultExtension: string;
      let filterName: string;
      
      switch (format) {
        case 'csv':
          content = exportToCSV(varsToExport);
          defaultExtension = 'csv';
          filterName = 'CSV Files';
          break;
        case 'json':
          content = JSON.stringify(varsToExport.map(v => ({ key: v.key, value: v.value })), null, 2);
          defaultExtension = 'json';
          filterName = 'JSON Files';
          break;
        case 'env':
        default:
          content = exportToEnv(varsToExport, options);
          defaultExtension = 'env';
          filterName = 'Environment Files';
          break;
      }
      
      const defaultFilename = generateExportFilename('project', environmentName, format);
      
      const filePath = await save({
        defaultPath: defaultFilename,
        filters: [{ name: filterName, extensions: [defaultExtension] }]
      });
      
      if (!filePath) return;

      // Write file using Tauri command
      await invoke('write_file_content', { filePath, content });
      const count = selectedVariables.size > 0 ? selectedVariables.size : variables.length;
      toast.success(`Exported ${count} variable${count !== 1 ? 's' : ''} as ${format.toUpperCase()}`);
      
      // Clear selection after export if in bulk mode
      if (selectedVariables.size > 0) {
        setSelectedVariables(new Set());
        setBulkMode(false);
      }
    } catch (err) {
      toast.error(err instanceof Error ? err.message : 'Export failed');
    }
  };

  const handleImport = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: 'Environment Files', extensions: ['env'] }]
      });

      if (!selected || Array.isArray(selected)) {
        return;
      }
      
      const selectedPath = selected; // selected is the path string
      console.log('Selected file path:', selectedPath);

      const fileContent = await invoke<string>('read_file_content', { filePath: selectedPath });
      console.log('File content loaded, length:', fileContent.length);

      const request: ImportEnvRequest = {
        environment_id: environmentId,
        content: fileContent
      };

      const response = await invoke<ImportEnvResponse>('import_env', { request });
      console.log('Import response:', response);

      if (response.success) {
        toast.success(`Imported ${response.imported_count} variables`);
        await loadVariables();
      } else {
        toast.error(response.message);
      }
    } catch (err) {
      console.error('Import error:', err);
      toast.error(err instanceof Error ? err.message : 'Import failed');
    }
  };

  // Keyboard shortcuts
  useKeyboardShortcuts([
    {
      key: 'v',
      ctrl: true,
      shift: true,
      description: 'New Variable',
      action: () => setShowCreateVar(true),
      disabled: showCreateVar || editingVar !== null,
    },
    {
      key: 'Escape',
      description: 'Close Modal/Clear Search',
      action: () => {
        if (showCreateVar) setShowCreateVar(false);
        else if (editingVar) setEditingVar(null);
        else if (deletingVar) setDeletingVar(null);
        else if (searchQuery) setSearchQuery('');
      },
    },
  ]);

  if (isLoading) {
    return <VariableTableSkeleton />;
  }

  if (error) {
    return (
      <div className="variable-list-state-container">
        <AlertTriangle size={40} className="variable-list-state-icon error" />
        <p>{error}</p>
        <button onClick={loadVariables} className="variable-list-btn-primary">Retry</button>
      </div>
    );
  }

  return (
    <div className="variable-list-container">
      <div className="variable-list-header">
        <div className="variable-list-search">
            <Search size={18} className="variable-list-search-icon" />
            <input
                type="text"
                placeholder="Search variables..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="variable-list-search-input"
                disabled={variables.length === 0}
            />
            {searchQuery && (
                <button onClick={() => setSearchQuery('')} className="variable-list-search-clear">
                    <X size={16} />
                </button>
            )}
        </div>
        <div className="variable-list-header-actions">
          <button 
            className={`variable-list-btn-secondary ${bulkMode ? 'active' : ''}`}
            onClick={() => {
              setBulkMode(!bulkMode);
              if (bulkMode) setSelectedVariables(new Set());
            }} 
            title="Toggle bulk selection mode"
            disabled={variables.length === 0}
          >
            <CheckSquare size={16} /> Select
          </button>
          <button className="variable-list-btn-secondary" onClick={handleImport} title="Import from .env file">
            <Upload size={16} /> Import
          </button>
          <button className="variable-list-btn-secondary" onClick={() => setShowExportModal(true)} title="Export variables" disabled={variables.length === 0}>
            <Download size={16} /> Export
          </button>
          <button className="variable-list-btn-primary" onClick={() => setShowCreateVar(true)}>
            <Plus size={16} /> Add Variable
          </button>
        </div>
      </div>

      {variables.length === 0 ? (
        <div className="variable-list-state-container empty">
          <Inbox size={56} className="variable-list-state-icon"/>
          <h3 className="variable-list-empty-title">No Variables Yet</h3>
          <p className="variable-list-empty-description">
            Start by creating your first environment variable. Variables store configuration values like API keys, database URLs, and more.
          </p>
          <div className="variable-list-empty-actions">
            <button onClick={() => setShowCreateVar(true)} className="variable-list-btn-primary">
              <Plus size={16} /> Create Variable
            </button>
            <button onClick={handleImport} className="variable-list-btn-secondary">
              <Upload size={16} /> Import from .env
            </button>
          </div>
        </div>
      ) : filteredVariables.length === 0 ? (
        <div className="variable-list-state-container empty">
          <Search size={56} className="variable-list-state-icon"/>
          <h3 className="variable-list-empty-title">No Results Found</h3>
          <p className="variable-list-empty-description">
            No variables match "{searchQuery}". Try a different search term or clear the filter.
          </p>
          <button onClick={() => setSearchQuery('')} className="variable-list-btn-secondary">
            <X size={16} /> Clear Search
          </button>
        </div>
      ) : (
        <>
          {bulkMode && selectedVariables.size > 0 && (
            <div className="variable-list-bulk-actions">
              <div className="variable-list-bulk-info">
                <CheckSquare size={18} />
                <span>{selectedVariables.size} variable{selectedVariables.size !== 1 ? 's' : ''} selected</span>
              </div>
              <div className="variable-list-bulk-buttons">
                <button onClick={handleBulkExport} className="variable-list-bulk-btn" title="Export selected">
                  <FileDown size={16} /> Export
                </button>
                <button onClick={handleBulkDelete} className="variable-list-bulk-btn danger" title="Delete selected">
                  <Trash size={16} /> Delete
                </button>
                <button onClick={deselectAllVariables} className="variable-list-bulk-btn" title="Clear selection">
                  <X size={16} /> Clear
                </button>
              </div>
            </div>
          )}
          <div className="variable-list-table-wrapper content-fade-in">
          <table className="variable-list-table">
            <thead>
              <tr>
                {bulkMode && (
                  <th className="checkbox-column">
                    <button
                      className="variable-list-checkbox-btn"
                      onClick={() => selectedVariables.size === filteredVariables.length ? deselectAllVariables() : selectAllVariables()}
                      title={selectedVariables.size === filteredVariables.length ? "Deselect all" : "Select all"}
                    >
                      {selectedVariables.size === filteredVariables.length ? <CheckSquare size={18} /> : <Square size={18} />}
                    </button>
                  </th>
                )}
                <th>Key</th>
                <th>Value</th>
                <th className="actions-column">Actions</th>
              </tr>
            </thead>
            <tbody>
              {filteredVariables.map((variable) => {
                const validation = validateVariable(variable.key, variable.value);
                const showValidation = validation.type !== 'none' && showValues[variable.id!];
                const isSelected = selectedVariables.has(variable.id!);
                
                return (
                <tr key={variable.id} className={isSelected ? 'selected' : ''}>
                  {bulkMode && (
                    <td className="checkbox-column">
                      <button
                        className="variable-list-checkbox-btn"
                        onClick={() => toggleSelectVariable(variable.id!)}
                        title={isSelected ? "Deselect" : "Select"}
                      >
                        {isSelected ? <CheckSquare size={18} /> : <Square size={18} />}
                      </button>
                    </td>
                  )}
                  <td className="variable-list-key">
                    <code 
                      className="clickable"
                      onClick={() => copyToClipboard(variable.key, `"${variable.key}" copied!`)}
                      title="Click to copy key"
                    >
                      {variable.key}
                    </code>
                  </td>
                  <td className="variable-list-value">
                    <div className="variable-list-value-container">
                      {showValues[variable.id!] ? (
                        <>
                          <code 
                            className="variable-list-value-text clickable" 
                            onClick={() => copyToClipboard(variable.value, `"${variable.key}" copied!`)}
                            title="Click to copy"
                          >
                            {variable.value}
                          </code>
                          {showValidation && (
                            <div className={`variable-validation-indicator ${validation.isValid ? 'valid' : 'invalid'}`} title={validation.isValid ? `Valid ${validation.type}` : validation.message}>
                              {validation.isValid ? (
                                <CheckCircle2 size={16} className="validation-icon" />
                              ) : (
                                <XCircle size={16} className="validation-icon" />
                              )}
                            </div>
                          )}
                        </>
                      ) : (
                        <span className="variable-list-value-hidden">••••••••••••••</span>
                      )}
                      <div className="variable-list-value-actions">
                        <button
                          className="variable-list-action-btn"
                          onClick={() => toggleShowValue(variable.id!)}
                          title={showValues[variable.id!] ? 'Hide value' : 'Show value'}
                        >
                          {showValues[variable.id!] ? <EyeOff size={16} /> : <Eye size={16} />}
                        </button>
                        <button
                          className="variable-list-action-btn"
                          onClick={() => copyToClipboard(variable.value)}
                          title="Copy to clipboard"
                        >
                          <Copy size={16} />
                        </button>
                      </div>
                    </div>
                  </td>
                  <td className="variable-list-actions">
                    <button className="variable-list-action-btn" onClick={() => setEditingVar(variable)} title="Edit">
                      <FilePenLine size={16} />
                    </button>
                    <button className="variable-list-action-btn danger" onClick={(e) => handleDeleteClick(variable, e)} title="Delete (Shift+Click to skip confirmation)">
                      <Trash2 size={16} />
                    </button>
                  </td>
                </tr>
                );
              })}
            </tbody>
          </table>
        </div>
        </>
      )}

      <VariableModal
        isOpen={showCreateVar}
        onClose={() => setShowCreateVar(false)}
        onSuccess={loadVariables}
        environmentId={environmentId}
      />

      {editingVar && (
        <VariableModal
          isOpen={true}
          onClose={() => setEditingVar(null)}
          onSuccess={loadVariables}
          environmentId={environmentId}
          variable={editingVar}
        />
      )}

      <DeleteConfirmModal
        isOpen={!!deletingVar}
        onClose={() => setDeletingVar(null)}
        onConfirm={confirmDelete}
        title="Delete Variable"
        message={`Are you sure you want to delete the variable "${deletingVar?.key || ''}"? This action cannot be undone.`}
        itemName={deletingVar?.key || ''}
      />

      <ExportModal
        isOpen={showExportModal}
        onClose={() => setShowExportModal(false)}
        onExport={handleExport}
        entityName={environmentName}
      />
    </div>
  );
};