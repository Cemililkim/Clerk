import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Environment, CreateEnvironmentRequest, CreateEnvironmentResponse, UpdateEnvironmentRequest, UpdateEnvironmentResponse } from '../types/database';
import { useToast } from './Toast';
import { Globe, FilePenLine, X, AlertTriangle } from 'lucide-react';
import './EnvironmentModal.css';

interface EnvironmentModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  projectId: number;
  environment?: Environment;
}

export const EnvironmentModal: React.FC<EnvironmentModalProps> = ({ 
  isOpen, 
  onClose, 
  onSuccess, 
  projectId,
  environment 
}) => {
  const [name, setName] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState('');
  const toast = useToast();

  const isEditMode = !!environment;

  useEffect(() => {
    if (isOpen) {
        if (environment) {
            setName(environment.name);
        } else {
            setName('');
        }
        setError('');
    }
  }, [environment, isOpen]);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!name.trim()) {
      setError('Environment name is required');
      return;
    }

    try {
      setIsSubmitting(true);
      if (isEditMode && environment) {
        const request: UpdateEnvironmentRequest = {
          id: environment.id!,
          project_id: projectId,
          name: name.trim(),
        };
        const response = await invoke<UpdateEnvironmentResponse>('update_environment', { request });
        if (response.success) {
          toast.success(`Environment "${name.trim()}" updated`);
          onSuccess();
          onClose();
        } else {
          setError(response.message);
        }
      } else {
        const request: CreateEnvironmentRequest = {
          project_id: projectId,
          name: name.trim(),
        };
        const response = await invoke<CreateEnvironmentResponse>('create_environment', { request });
        if (response.success) {
          toast.success(`Environment "${name.trim()}" created`);
          onSuccess();
          onClose();
        } else {
          setError(response.message);
        }
      }
    } catch (err) {
      const action = isEditMode ? 'update' : 'create';
      console.error(`Failed to ${action} environment:`, err);
      setError(err instanceof Error ? err.message : `Failed to ${action} environment`);
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
    <div className="environment-modal-overlay" onClick={handleOverlayClick}>
      <div className="environment-modal-content">
        <div className="environment-modal-header">
          <h2>
            {isEditMode ? <FilePenLine size={20} /> : <Globe size={20} />}
            {isEditMode ? 'Edit Environment' : 'Create New Environment'}
          </h2>
          <button 
            className="environment-modal-close"
            onClick={onClose}
            disabled={isSubmitting}
            aria-label="Close modal"
          >
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="environment-modal-body">
          <div className="environment-modal-form-group">
            <label htmlFor="env-name">
              Environment Name <span className="environment-modal-required">*</span>
            </label>
            <input
              id="env-name"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., development, production"
              disabled={isSubmitting}
              autoFocus
              required
            />
            <p className="environment-modal-hint">
              Common names: development, staging, production, testing
            </p>
          </div>

          {error && (
            <div className="environment-modal-error-message">
              <AlertTriangle size={16} />
              {error}
            </div>
          )}

          <div className="environment-modal-footer">
            <button
              type="button"
              className="environment-modal-btn-cancel"
              onClick={onClose}
              disabled={isSubmitting}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="environment-modal-btn-submit"
              disabled={isSubmitting || !name.trim()}
            >
              {isSubmitting ? (isEditMode ? 'Updating...' : 'Creating...') : (isEditMode ? 'Update Environment' : 'Create Environment')}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};