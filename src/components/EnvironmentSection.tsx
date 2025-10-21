import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { 
  Environment, 
  GetEnvironmentsRequest, 
  GetEnvironmentsResponse,
  DeleteEnvironmentRequest,
  DeleteEnvironmentResponse 
} from '../types/database';
import { EnvironmentModal } from './EnvironmentModal';
import { DeleteConfirmModal } from './DeleteConfirmModal';
import { VariableList } from './VariableList';
import { useToast } from './Toast';
import { useKeyboardShortcuts } from '../hooks/useKeyboardShortcuts';
import { Globe, Plus, FilePenLine, Trash2, AlertTriangle, Inbox } from 'lucide-react';
import { EnvironmentTabsSkeleton } from './Skeleton';
import './EnvironmentSection.css';

interface EnvironmentSectionProps {
  projectId: number;
}

export const EnvironmentSection: React.FC<EnvironmentSectionProps> = ({ projectId }) => {
  const [environments, setEnvironments] = useState<Environment[]>([]);
  const [selectedEnv, setSelectedEnv] = useState<Environment | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');
  const [showCreateEnv, setShowCreateEnv] = useState(false);
  const [editingEnv, setEditingEnv] = useState<Environment | null>(null);
  const [deletingEnv, setDeletingEnv] = useState<Environment | null>(null);
  const toast = useToast();

  const loadEnvironments = async () => {
    try {
      setIsLoading(true);
      setError('');
      
      const request: GetEnvironmentsRequest = { project_id: projectId };
      const response = await invoke<GetEnvironmentsResponse>('get_environments', { request });

      if (response.success) {
        setEnvironments(response.environments);
        
        // If there's no currently selected environment, or the selected one no longer exists
        const currentSelectedId = selectedEnv?.id;
        const selectedStillExists = response.environments.some(e => e.id === currentSelectedId);

        if (!selectedStillExists && response.environments.length > 0) {
            setSelectedEnv(response.environments[0] || null);
        } else if (response.environments.length === 0) {
            setSelectedEnv(null);
        }
      } else {
        setError(response.message);
      }
    } catch (err) {
      console.error('Failed to load environments:', err);
      setError(err instanceof Error ? err.message : 'Failed to load environments');
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    // Reset selection when project changes
    setSelectedEnv(null);
    loadEnvironments();
  }, [projectId]);

  const handleDeleteClick = (env: Environment, event: React.MouseEvent) => {
    event.stopPropagation();
    if (event.shiftKey) {
      // Shift tuşuna basılıysa direkt sil
      deleteEnvironment(env);
    } else {
      // Normal durumda modal aç
      setDeletingEnv(env);
    }
  };

  const deleteEnvironment = async (env: Environment) => {
    try {
        const request: DeleteEnvironmentRequest = { id: env.id! };
        const response = await invoke<DeleteEnvironmentResponse>('delete_environment', { request });
        if (response.success) {
            toast.success(`Environment "${env.name}" deleted`);
            await loadEnvironments();
        } else {
            throw new Error(response.message);
        }
    } catch(err) {
        toast.error(err instanceof Error ? err.message : 'Failed to delete environment');
    }
  };

  const confirmDelete = async () => {
    if (!deletingEnv) return;
    await deleteEnvironment(deletingEnv);
  };

  // Keyboard shortcuts
  useKeyboardShortcuts([
    {
      key: 'e',
      ctrl: true,
      description: 'New Environment',
      action: () => setShowCreateEnv(true),
      disabled: showCreateEnv || editingEnv !== null,
    },
    {
      key: 'Escape',
      description: 'Close Modal',
      action: () => {
        if (showCreateEnv) setShowCreateEnv(false);
        else if (editingEnv) setEditingEnv(null);
        else if (deletingEnv) setDeletingEnv(null);
      },
    },
  ]);

  if (isLoading) {
    return <EnvironmentTabsSkeleton />;
  }

  if (error) {
    return (
      <div className="environment-section-state-container">
        <AlertTriangle size={40} className="environment-section-state-icon error" />
        <p>{error}</p>
        <button onClick={loadEnvironments} className="environment-section-btn-primary">Retry</button>
      </div>
    );
  }

  return (
    <div className="environment-section-container">
      <div className="environment-section-header">
        <h3><Globe size={20} /> Environments</h3>
        <button 
          className="environment-section-btn-add"
          onClick={() => setShowCreateEnv(true)}
        >
          <Plus size={16} /> Add Environment
        </button>
      </div>

      {environments.length === 0 ? (
        <div className="environment-section-state-container empty">
          <Inbox size={56} className="environment-section-state-icon" />
          <h3 className="environment-section-empty-title">No Environments</h3>
          <p className="environment-section-empty-description">
            Environments help you organize variables for different deployment stages like Development, Staging, and Production.
          </p>
          <button 
            onClick={() => setShowCreateEnv(true)}
            className="environment-section-btn-primary"
          >
            <Plus size={16} /> Create Environment
          </button>
        </div>
      ) : (
        <div className="environment-section-content content-fade-in">
          <div className="environment-section-tabs">
            {environments.map((env) => (
              <div
                key={env.id}
                className={`environment-section-tab ${selectedEnv?.id === env.id ? 'active' : ''}`}
                onClick={() => setSelectedEnv(env)}
              >
                <span className="environment-section-tab-name">{env.name}</span>
                <div className="environment-section-tab-actions">
                  <button
                    className="environment-section-action-btn"
                    onClick={(e) => { e.stopPropagation(); setEditingEnv(env); }}
                    title="Edit Environment"
                  >
                    <FilePenLine size={16} />
                  </button>
                  <button
                    className="environment-section-action-btn danger"
                    onClick={(e) => handleDeleteClick(env, e)}
                    title="Delete Environment (Shift+Click to skip confirmation)"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              </div>
            ))}
          </div>
          {selectedEnv && (
            <VariableList 
              key={selectedEnv.id} // Add key to force re-mount on env change
              environmentId={selectedEnv.id!} 
              environmentName={selectedEnv.name}
            />
          )}
        </div>
      )}

      <EnvironmentModal
        isOpen={showCreateEnv}
        onClose={() => setShowCreateEnv(false)}
        onSuccess={loadEnvironments}
        projectId={projectId}
      />

      {editingEnv && (
        <EnvironmentModal
          isOpen={true}
          onClose={() => setEditingEnv(null)}
          onSuccess={async () => {
            const currentId = editingEnv.id;
            await loadEnvironments();
            setEnvironments(prev => {
                const updatedEnv = prev.find(e => e.id === currentId);
                if (updatedEnv) {
                    setSelectedEnv(updatedEnv);
                }
                return prev;
            })
          }}
          projectId={projectId}
          environment={editingEnv}
        />
      )}

      <DeleteConfirmModal
        isOpen={!!deletingEnv}
        onClose={() => setDeletingEnv(null)}
        onConfirm={confirmDelete}
        title="Delete Environment"
        message={`Are you sure you want to delete the "${deletingEnv?.name || ''}" environment? All associated variables will be permanently removed.`}
        itemName={deletingEnv?.name || ''}
      />
    </div>
  );
};