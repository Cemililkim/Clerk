import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Project, CreateProjectRequest, CreateProjectResponse, UpdateProjectRequest, UpdateProjectResponse } from '../types/database';
import { useToast } from './Toast';
import { PackagePlus, FilePenLine, X, AlertTriangle } from 'lucide-react';
import './ProjectModal.css';

interface ProjectModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  project?: Project;
}

export const ProjectModal: React.FC<ProjectModalProps> = ({ isOpen, onClose, onSuccess, project }) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState('');
  const toast = useToast();

  const isEditMode = !!project;

  useEffect(() => {
    if (isOpen) {
        if (project) {
            setName(project.name);
            setDescription(project.description || '');
        } else {
            setName('');
            setDescription('');
        }
        setError('');
    }
  }, [project, isOpen]);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!name.trim()) {
      setError('Project name is required');
      return;
    }

    try {
      setIsSubmitting(true);
      if (isEditMode && project) {
        const request: UpdateProjectRequest = {
          id: project.id!,
          name: name.trim(),
          ...(description.trim() && { description: description.trim() }),
        };
        const response = await invoke<UpdateProjectResponse>('update_project', { request });
        if (response.success) {
          toast.success(`Project "${name.trim()}" updated`);
          onSuccess();
          onClose();
        } else {
          setError(response.message);
        }
      } else {
        const request: CreateProjectRequest = {
          name: name.trim(),
          ...(description.trim() && { description: description.trim() }),
        };
        const response = await invoke<CreateProjectResponse>('create_project', { request });
        if (response.success) {
          toast.success(`Project "${name.trim()}" created`);
          onSuccess();
          onClose();
        } else {
          setError(response.message);
        }
      }
    } catch (err) {
      const action = isEditMode ? 'update' : 'create';
      console.error(`Failed to ${action} project:`, err);
      setError(err instanceof Error ? err.message : `Failed to ${action} project`);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  return (
    <div className="project-modal-overlay" onClick={handleOverlayClick}>
      <div className="project-modal-content">
        <div className="project-modal-header">
          <h2>
            {isEditMode ? <FilePenLine size={20} /> : <PackagePlus size={20} />}
            {isEditMode ? 'Edit Project' : 'Create New Project'}
          </h2>
          <button 
            className="project-modal-close"
            onClick={onClose}
            disabled={isSubmitting}
            aria-label="Close modal"
          >
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="project-modal-body">
          <div className="project-modal-form-group">
            <label htmlFor="project-name">
              Project Name <span className="project-modal-required">*</span>
            </label>
            <input
              id="project-name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., My Web App"
              disabled={isSubmitting}
              autoFocus
              required
            />
          </div>

          <div className="project-modal-form-group">
            <label htmlFor="project-description">
              Description <span className="project-modal-optional">(optional)</span>
            </label>
            <textarea
              id="project-description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="A brief description of your project..."
              rows={3}
              disabled={isSubmitting}
            />
          </div>

          {error && (
            <div className="project-modal-error-message">
              <AlertTriangle size={16} />
              {error}
            </div>
          )}

          <div className="project-modal-footer">
            <button
              type="button"
              className="project-modal-btn-cancel"
              onClick={onClose}
              disabled={isSubmitting}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="project-modal-btn-submit"
              disabled={isSubmitting || !name.trim()}
            >
              {isSubmitting ? (isEditMode ? 'Updating...' : 'Creating...') : (isEditMode ? 'Update Project' : 'Create Project')}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};