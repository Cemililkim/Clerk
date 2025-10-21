import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Project, GetProjectsResponse, DeleteProjectRequest, DeleteProjectResponse } from '../types/database';
import { ProjectModal } from './ProjectModal';
import { DeleteConfirmModal } from './DeleteConfirmModal';
import { EnvironmentSection } from './EnvironmentSection';
import { SettingsModal } from './SettingsModal';
import AuditLog from './AuditLog';
import { useToast } from './Toast';
import { useInactivityTimer } from '../hooks/useInactivityTimer';
import { useKeyboardShortcuts } from '../hooks/useKeyboardShortcuts';
import { useDebounce } from '../hooks/useDebounce';
import { 
  ShieldCheck, RefreshCw, ScrollText, Settings, Lock, Plus, Search, 
  Package, Rocket, FilePenLine, Trash2, X, AlertTriangle, Inbox 
} from 'lucide-react';
import { ProjectListSkeleton } from './Skeleton';
import './VaultDashboard.css';

interface VaultDashboardProps {
  onLock: () => void;
}

export const VaultDashboard: React.FC<VaultDashboardProps> = ({ onLock }) => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');
  const [showCreateProject, setShowCreateProject] = useState(false);
  const [editingProject, setEditingProject] = useState<Project | null>(null);
  const [deletingProject, setDeletingProject] = useState<Project | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [lockTimeout, setLockTimeout] = useState(0);
  const [showSettings, setShowSettings] = useState(false);
  const [showAuditLog, setShowAuditLog] = useState(false);
  const toast = useToast();

  const handleInactivityTimeout = () => {
    toast.info('Locking vault due to inactivity...');
    setTimeout(async () => {
      try {
        await invoke('lock_vault');
        onLock();
      } catch (err) {
        console.error('Failed to lock vault:', err);
        toast.error('Failed to lock vault');
      }
    }, 1000);
  };

  useInactivityTimer({
    timeout: lockTimeout * 60 * 1000,
    onTimeout: handleInactivityTimeout,
    enabled: lockTimeout > 0,
  });

  useEffect(() => {
    const loadLockTimeout = async () => {
      try {
        const timeout = await invoke<number>('get_lock_timeout');
        setLockTimeout(timeout);
      } catch (err) {
        console.error('Failed to load lock timeout:', err);
        setLockTimeout(0);
      }
    };
    loadLockTimeout();
  }, []);

  // Debounce search query to avoid excessive filtering
  const debouncedSearchQuery = useDebounce(searchQuery, 300);

  // Memoize filtered projects to avoid unnecessary re-computation
  const filteredProjects = useMemo(() => {
    return projects.filter(project =>
      project.name.toLowerCase().includes(debouncedSearchQuery.toLowerCase()) ||
      (project.description && project.description.toLowerCase().includes(debouncedSearchQuery.toLowerCase()))
    );
  }, [projects, debouncedSearchQuery]);

  const loadProjects = useCallback(async () => {
    try {
      setIsLoading(true);
      setError('');
      const response = await invoke<GetProjectsResponse>('get_projects');
      if (response.success) {
        setProjects(response.projects);
        if (!selectedProject && response.projects.length > 0) {
            const firstProject = response.projects[0];
            if (firstProject) {
                setSelectedProject(firstProject);
            }
        } else if (response.projects.length === 0) {
            setSelectedProject(null);
        }
      } else {
        setError(response.message);
      }
    } catch (err) {
      console.error('Failed to load projects:', err);
      setError(err instanceof Error ? err.message : 'Failed to load projects');
    } finally {
      setIsLoading(false);
    }
  }, [selectedProject]);
  
  useEffect(() => {
    loadProjects();
  }, [loadProjects]);

  const handleLock = useCallback(async () => {
    try {
      await invoke('lock_vault');
      onLock();
    } catch (err) {
      console.error('Failed to lock vault:', err);
      toast.error('Failed to lock vault');
    }
  }, [onLock, toast]);

  const deleteProject = useCallback(async (project: Project) => {
    try {
        const request: DeleteProjectRequest = { id: project.id! };
        const response = await invoke<DeleteProjectResponse>('delete_project', { request });
        if (response.success) {
            toast.success(`Project "${project.name}" deleted successfully`);
            if (selectedProject?.id === project.id) {
                setSelectedProject(null);
            }
            await loadProjects();
        } else {
            throw new Error(response.message);
        }
    } catch(err) {
        toast.error(err instanceof Error ? err.message : 'Failed to delete project');
    }
  }, [toast, selectedProject, loadProjects]);

  const handleDeleteClick = useCallback((project: Project, event: React.MouseEvent) => {
    if (event.shiftKey) {
      // Shift tuşuna basılıysa direkt sil
      deleteProject(project);
    } else {
      // Normal durumda modal aç
      setDeletingProject(project);
    }
  }, [deleteProject]);

  const confirmDelete = useCallback(async () => {
    if (!deletingProject) return;
    try {
        const request: DeleteProjectRequest = { id: deletingProject.id! };
        const response = await invoke<DeleteProjectResponse>('delete_project', { request });
        if (response.success) {
            toast.success(`Project "${deletingProject.name}" deleted successfully`);
            if (selectedProject?.id === deletingProject.id) {
                setSelectedProject(null);
            }
            await loadProjects();
        } else {
            throw new Error(response.message);
        }
    } catch (err) {
        toast.error(err instanceof Error ? err.message : 'Failed to delete project');
    }
  }, [deletingProject, toast, selectedProject, loadProjects]);

  const handleSettingsClose = useCallback(() => {
      setShowSettings(false);
      const reloadTimeout = async () => {
          try {
              const timeout = await invoke<number>('get_lock_timeout');
              setLockTimeout(timeout);
          } catch(err) {
              console.error('Failed to reload lock timeout', err);
          }
      };
      reloadTimeout();
  }, []);

  // Keyboard shortcuts
  useKeyboardShortcuts([
    {
      key: 'n',
      ctrl: true,
      description: 'New Project',
      action: () => setShowCreateProject(true),
      disabled: showCreateProject || editingProject !== null || showSettings || showAuditLog,
    },
    {
      key: ',',
      ctrl: true,
      description: 'Settings',
      action: () => setShowSettings(true),
      disabled: showSettings,
    },
    {
      key: 'l',
      ctrl: true,
      shift: true,
      description: 'Lock Vault',
      action: handleLock,
    },
    {
      key: 'a',
      ctrl: true,
      shift: true,
      description: 'Audit Log',
      action: () => setShowAuditLog(true),
      disabled: showAuditLog,
    },
    {
      key: 'r',
      ctrl: true,
      description: 'Refresh Projects',
      action: loadProjects,
    },
    {
      key: 'f',
      ctrl: true,
      description: 'Focus Search',
      action: () => {
        const searchInput = document.querySelector('.vault-dashboard-search-input') as HTMLInputElement;
        searchInput?.focus();
      },
    },
    {
      key: 'Escape',
      description: 'Close Modal',
      action: () => {
        if (showCreateProject) setShowCreateProject(false);
        else if (editingProject) setEditingProject(null);
        else if (deletingProject) setDeletingProject(null);
        else if (showSettings) setShowSettings(false);
        else if (showAuditLog) setShowAuditLog(false);
      },
    },
  ]);

  return (
    <div className="vault-dashboard">
      <header className="vault-dashboard-header">
        <div className="vault-dashboard-header-left">
          <ShieldCheck size={32} className="vault-dashboard-logo-icon" />
          <h1>Clerk</h1>
        </div>
        <div className="vault-dashboard-header-right">
          <button className="vault-dashboard-btn-icon" onClick={loadProjects} title="Refresh">
            <RefreshCw size={18} />
          </button>
          <button className="vault-dashboard-btn-icon" onClick={() => setShowAuditLog(true)} title="Audit Log">
            <ScrollText size={18} />
          </button>
          <button className="vault-dashboard-btn-icon" onClick={() => setShowSettings(true)} title="Settings">
            <Settings size={18} />
          </button>
          <button className="vault-dashboard-btn-lock" onClick={handleLock}>
            <Lock size={16} /> Lock Vault
          </button>
        </div>
      </header>

      <div className="vault-dashboard-body">
        <aside className="vault-dashboard-sidebar">
          <div className="vault-dashboard-sidebar-header">
            <h2>Projects</h2>
            <button 
              className="vault-dashboard-btn-add"
              onClick={() => setShowCreateProject(true)}
              title="Create Project"
            >
              <Plus size={20} />
            </button>
          </div>
          <div className="vault-dashboard-sidebar-search">
            <Search size={16} className="vault-dashboard-search-icon" />
            <input
              type="text"
              placeholder="Search projects..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="vault-dashboard-search-input"
            />
            {searchQuery && (
              <button onClick={() => setSearchQuery('')} className="vault-dashboard-search-clear">
                  <X size={16} />
              </button>
            )}
          </div>

          <div className="vault-dashboard-project-list">
            {isLoading ? (
              <ProjectListSkeleton />
            ) : error ? (
              <div className="vault-dashboard-state-container">
                <AlertTriangle size={40} className="vault-dashboard-state-icon error"/>
                <p>{error}</p>
                <button onClick={loadProjects} className="vault-dashboard-btn-primary">
                  Retry
                </button>
              </div>
            ) : filteredProjects.length === 0 ? (
              <div className="vault-dashboard-state-container">
                <Inbox size={56} className="vault-dashboard-state-icon"/>
                {projects.length === 0 ? (
                  <>
                    <h3 className="vault-dashboard-empty-title">No Projects Yet</h3>
                    <p className="vault-dashboard-empty-description">
                      Projects help you organize your environment variables by application or service. Start by creating your first project.
                    </p>
                    <button onClick={() => setShowCreateProject(true)} className="vault-dashboard-btn-primary">
                      <Plus size={16} /> Create Project
                    </button>
                  </>
                ) : (
                  <>
                    <h3 className="vault-dashboard-empty-title">No Results Found</h3>
                    <p className="vault-dashboard-empty-description">
                      No projects match "{searchQuery}". Try a different search term.
                    </p>
                    <button onClick={() => setSearchQuery('')} className="vault-dashboard-btn-primary">
                      <X size={16} /> Clear Search
                    </button>
                  </>
                )}
              </div>
            ) : (
              filteredProjects.map((project) => (
                <div
                  key={project.id}
                  className={`vault-dashboard-project-item content-fade-in ${selectedProject?.id === project.id ? 'selected' : ''}`}
                  onClick={() => setSelectedProject(project)}
                >
                  <Package size={24} className="vault-dashboard-project-icon" />
                  <div className="vault-dashboard-project-info">
                    <h3>{project.name}</h3>
                    {project.description && (
                      <p className="vault-dashboard-project-description">{project.description}</p>
                    )}
                  </div>
                </div>
              ))
            )}
          </div>
        </aside>

        <main className="vault-dashboard-main-content">
          {!selectedProject ? (
            <div className="vault-dashboard-welcome-content">
              <Rocket size={64} className="vault-dashboard-welcome-icon" />
              <h2>Welcome to Clerk!</h2>
              <p>Select a project from the sidebar or create a new one to get started.</p>
            </div>
          ) : (
            <div className="vault-dashboard-project-content">
              <div className="vault-dashboard-content-header">
                <div className="vault-dashboard-content-title">
                  <h2>{selectedProject.name}</h2>
                  {selectedProject.description && (
                    <p className="vault-dashboard-content-subtitle">{selectedProject.description}</p>
                  )}
                </div>
                <div className="vault-dashboard-content-actions">
                  <button className="vault-dashboard-btn-secondary" onClick={() => setEditingProject(selectedProject)}>
                    <FilePenLine size={16} /> Edit
                  </button>
                  <button className="vault-dashboard-btn-danger" onClick={(e) => handleDeleteClick(selectedProject, e)} title="Delete (Shift+Click to skip confirmation)">
                    <Trash2 size={16} /> Delete
                  </button>
                </div>
              </div>
              <EnvironmentSection projectId={selectedProject.id!} />
            </div>
          )}
        </main>
      </div>

      <ProjectModal
        isOpen={showCreateProject}
        onClose={() => setShowCreateProject(false)}
        onSuccess={loadProjects}
      />

      {editingProject && (
        <ProjectModal
          isOpen={true}
          onClose={() => setEditingProject(null)}
          onSuccess={async () => {
            const currentId = editingProject.id;
            await loadProjects();
            // After reloading, find the updated project and re-select it
            setProjects(prevProjects => {
                const updatedProject = prevProjects.find(p => p.id === currentId);
                if (updatedProject) {
                    setSelectedProject(updatedProject);
                }
                return prevProjects;
            });
          }}
          project={editingProject}
        />
      )}

      <DeleteConfirmModal
        isOpen={!!deletingProject}
        onClose={() => setDeletingProject(null)}
        onConfirm={confirmDelete}
        title="Delete Project"
        message={`Are you sure you want to delete this project? All associated environments and variables will be permanently removed.`}
        itemName={deletingProject?.name || ''}
      />

      <SettingsModal 
        isOpen={showSettings} 
        onClose={handleSettingsClose}
        onRestoreSuccess={() => {
          // Reload all projects after restore
          loadProjects();
          toast.success('Backup restored successfully! Data refreshed.');
        }}
      />

      {showAuditLog && <AuditLog onClose={() => setShowAuditLog(false)} />}
    </div>
  );
};