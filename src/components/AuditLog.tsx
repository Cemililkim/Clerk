import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { 
    ScrollText, X, FileText, Braces, SlidersHorizontal, ChevronDown, Check,
    Package, Globe, KeyRound, PlusCircle, FilePenLine, Trash2, Inbox 
} from 'lucide-react';
import './AuditLog.css';

// Interfaces remain the same...
interface AuditLogEntry {
  id: number;
  timestamp: number;
  operation_type: string;
  entity_type: string;
  entity_id: number | null;
  entity_name: string | null;
  details: string | null;
  created_at: number;
}
interface AuditLogFilter {
  entity_type?: string;
  operation_type?: string;
  limit?: number;
  offset?: number;
}
interface AuditLogProps {
  onClose: () => void;
}


// Helper components remain the same...
const OperationBadge: React.FC<{ type: string }> = ({ type }) => {
    let icon;
    switch (type.toLowerCase()) {
        case 'create': icon = <PlusCircle size={14} />; break;
        case 'update': icon = <FilePenLine size={14} />; break;
        case 'delete': icon = <Trash2 size={14} />; break;
        default: icon = null;
    }
    return (
        <span className={`audit-log-operation-badge operation-${type.toLowerCase()}`}>
            {icon}
            {type}
        </span>
    );
};
const EntityBadge: React.FC<{ type: string }> = ({ type }) => {
    let icon;
    switch (type.toLowerCase()) {
        case 'project': icon = <Package size={14} />; break;
        case 'environment': icon = <Globe size={14} />; break;
        case 'variable': icon = <KeyRound size={14} />; break;
        default: icon = null;
    }
    return (
        <span className="audit-log-entity-badge">
            {icon}
            {type}
        </span>
    );
};


export const AuditLog: React.FC<AuditLogProps> = ({ onClose }) => {
  const [logs, setLogs] = useState<AuditLogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentPage, setCurrentPage] = useState(1);
  const [filterEntityType, setFilterEntityType] = useState<string>('');
  const [filterOperationType, setFilterOperationType] = useState<string>('');
  const [showFilters, setShowFilters] = useState(false);
  
  // New states for custom dropdowns
  const [isEntityTypeOpen, setIsEntityTypeOpen] = useState(false);
  const [isOperationTypeOpen, setIsOperationTypeOpen] = useState(false);
  
  const itemsPerPage = 50;

  // Constants for dropdown options
  const ENTITY_OPTIONS = [
    { value: '', label: 'All' },
    { value: 'project', label: 'Project' },
    { value: 'environment', label: 'Environment' },
    { value: 'variable', label: 'Variable' },
  ];
  const OPERATION_OPTIONS = [
    { value: '', label: 'All' },
    { value: 'create', label: 'Create' },
    { value: 'update', label: 'Update' },
    { value: 'delete', label: 'Delete' },
  ];

  useEffect(() => {
    loadLogs();
  }, [currentPage, filterEntityType, filterOperationType]);

  // Rest of the functions (loadLogs, formatTimestamp, etc.) remain the same...
  const loadLogs = async () => {
    try {
      setLoading(true);
      const filter: AuditLogFilter = {
        limit: itemsPerPage,
        offset: (currentPage - 1) * itemsPerPage,
        ...(filterEntityType && { entity_type: filterEntityType }),
        ...(filterOperationType && { operation_type: filterOperationType }),
      };
      const result = await invoke<AuditLogEntry[]>('get_audit_logs', { filter });
      setLogs(result);
    } catch (error) {
      console.error('Failed to load audit logs:', error);
      alert('Failed to load audit logs');
    } finally {
      setLoading(false);
    }
  };

  const formatTimestamp = (timestamp: number): string => {
    return new Date(timestamp * 1000).toLocaleString('en-US', {
      year: 'numeric', month: 'short', day: 'numeric',
      hour: '2-digit', minute: '2-digit', second: '2-digit',
    });
  };

  const parseDetails = (details: string | null): any => {
    if (!details) return null;
    try {
      return JSON.parse(details);
    } catch { return null; }
  };

  const clearFilters = () => {
    setFilterEntityType('');
    setFilterOperationType('');
    setCurrentPage(1);
  };

  const exportLogs = async (format: 'csv' | 'json') => {
    try {
      const filePath = await save({
        defaultPath: `audit-log-${new Date().toISOString().split('T')[0]}.${format}`,
        filters: [{ name: `${format.toUpperCase()} Files`, extensions: [format] }]
      });
      if (!filePath) return;

      const filter: AuditLogFilter = {};
      if (filterEntityType) filter.entity_type = filterEntityType;
      if (filterOperationType) filter.operation_type = filterOperationType;

      const command = `export_audit_logs_${format}`;
      const result = await invoke<string>(command, {
        filter: Object.keys(filter).length > 0 ? filter : null,
        filePath
      });
      alert(result);
    } catch (error) {
      console.error(`Failed to export ${format}:`, error);
      alert(`Failed to export audit logs to ${format}: ${error}`);
    }
  };


  const hasActiveFilters = filterEntityType || filterOperationType;

  return (
    <div className="audit-log-overlay" onClick={onClose}>
      <div className="audit-log-modal" onClick={(e) => e.stopPropagation()}>
        <div className="audit-log-header">
          <h2><ScrollText size={20} /> Audit Log</h2>
          <button className="audit-log-close-button" onClick={onClose}><X size={20} /></button>
        </div>

        <div className="audit-log-controls">
          <div className="audit-log-controls-left">
            <button 
              className={`audit-log-filter-toggle ${hasActiveFilters ? 'active' : ''}`}
              onClick={() => setShowFilters(!showFilters)}
            >
              <SlidersHorizontal size={16} /> Filters <ChevronDown size={16} className={`audit-log-chevron ${showFilters ? 'open' : ''}`} />
            </button>
          </div>
          <div className="audit-log-controls-right">
            <button className="audit-log-btn-secondary" onClick={() => exportLogs('csv')} title="Export as CSV">
              <FileText size={16} /> Export CSV
            </button>
            <button className="audit-log-btn-secondary" onClick={() => exportLogs('json')} title="Export as JSON">
              <Braces size={16} /> Export JSON
            </button>
          </div>
        </div>

        {showFilters && (
          <div className="audit-log-filters-panel">
            {/* --- CUSTOM DROPDOWN FOR ENTITY TYPE --- */}
            <div className="audit-log-filter-group">
              <label>Entity Type:</label>
              <div className="audit-log-custom-select">
                <button type="button" className="audit-log-select-button" onClick={() => setIsEntityTypeOpen(!isEntityTypeOpen)}>
                  <span>{ENTITY_OPTIONS.find(o => o.value === filterEntityType)?.label}</span>
                  <ChevronDown size={16} className={`audit-log-select-arrow ${isEntityTypeOpen ? 'open' : ''}`} />
                </button>
                {isEntityTypeOpen && (
                  <>
                    <div className="audit-log-select-backdrop" onClick={() => setIsEntityTypeOpen(false)} />
                    <div className="audit-log-select-options">
                      {ENTITY_OPTIONS.map(option => (
                        <button key={option.value} type="button" className={`audit-log-select-option ${filterEntityType === option.value ? 'selected' : ''}`} onClick={() => { setFilterEntityType(option.value); setIsEntityTypeOpen(false); setCurrentPage(1); }}>
                          {option.label}
                          {filterEntityType === option.value && <Check size={16} />}
                        </button>
                      ))}
                    </div>
                  </>
                )}
              </div>
            </div>

            {/* --- CUSTOM DROPDOWN FOR OPERATION TYPE --- */}
            <div className="audit-log-filter-group">
              <label>Operation:</label>
              <div className="audit-log-custom-select">
                <button type="button" className="audit-log-select-button" onClick={() => setIsOperationTypeOpen(!isOperationTypeOpen)}>
                  <span>{OPERATION_OPTIONS.find(o => o.value === filterOperationType)?.label}</span>
                  <ChevronDown size={16} className={`audit-log-select-arrow ${isOperationTypeOpen ? 'open' : ''}`} />
                </button>
                {isOperationTypeOpen && (
                  <>
                    <div className="audit-log-select-backdrop" onClick={() => setIsOperationTypeOpen(false)} />
                    <div className="audit-log-select-options">
                      {OPERATION_OPTIONS.map(option => (
                        <button key={option.value} type="button" className={`audit-log-select-option ${filterOperationType === option.value ? 'selected' : ''}`} onClick={() => { setFilterOperationType(option.value); setIsOperationTypeOpen(false); setCurrentPage(1); }}>
                          {option.label}
                          {filterOperationType === option.value && <Check size={16} />}
                        </button>
                      ))}
                    </div>
                  </>
                )}
              </div>
            </div>

            {hasActiveFilters && (
              <button className="audit-log-clear-filters-button" onClick={clearFilters}>
                Clear Filters
              </button>
            )}
          </div>
        )}

        {/* Rest of the component (content, table, footer) remains the same */}
        <div className="audit-log-content">
          {loading ? (
            <div className="audit-log-state-container">
              <div className="audit-log-spinner"></div>
              <p>Loading audit logs...</p>
            </div>
          ) : logs.length === 0 ? (
            <div className="audit-log-state-container">
              <Inbox size={48} className="audit-log-state-icon" />
              <p>No audit logs found.</p>
              {hasActiveFilters && (
                <button className="audit-log-btn-primary" onClick={clearFilters}>
                  Clear Filters
                </button>
              )}
            </div>
          ) : (
            <div className="audit-log-table-container">
              <table className="audit-log-table">
                <thead>
                  <tr>
                    <th>Timestamp</th>
                    <th>Operation</th>
                    <th>Entity</th>
                    <th>Name</th>
                    <th>Details</th>
                  </tr>
                </thead>
                <tbody>
                  {logs.map((log) => {
                    const details = parseDetails(log.details);
                    return (
                      <tr key={log.id}>
                        <td className="audit-log-timestamp-cell">{formatTimestamp(log.timestamp)}</td>
                        <td><OperationBadge type={log.operation_type} /></td>
                        <td><EntityBadge type={log.entity_type} /></td>
                        <td className="audit-log-name-cell">{log.entity_name || <span className="audit-log-text-muted">—</span>}</td>
                        <td className="audit-log-details-cell">
                          {details ? (
                            <details>
                              <summary>View Details</summary>
                              <pre>{JSON.stringify(details, null, 2)}</pre>
                            </details>
                          ) : (
                            <span className="audit-log-text-muted">—</span>
                          )}
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          )}
        </div>

        <div className="audit-log-footer">
          <span className="audit-log-count">
            Showing {logs.length} entries on this page
          </span>
          <div className="audit-log-pagination">
            <button onClick={() => setCurrentPage(p => Math.max(1, p - 1))} disabled={currentPage === 1 || loading}>
              Previous
            </button>
            <span>Page {currentPage}</span>
            <button onClick={() => setCurrentPage(p => p + 1)} disabled={logs.length < itemsPerPage || loading}>
              Next
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AuditLog;