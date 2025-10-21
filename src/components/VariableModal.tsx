import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Variable, CreateVariableRequest, CreateVariableResponse, UpdateVariableRequest, UpdateVariableResponse } from '../types/database';
import { useToast } from './Toast';
import { validateVariable } from '../utils/variableValidation';
import { KeyRound, X, AlertTriangle, Lock, CheckCircle2, XCircle } from 'lucide-react';
import './VariableModal.css';

interface VariableModalProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  environmentId: number;
  variable?: Variable;
}

export const VariableModal: React.FC<VariableModalProps> = ({ 
  isOpen, 
  onClose, 
  onSuccess, 
  environmentId,
  variable 
}) => {
  const [key, setKey] = useState('');
  const [value, setValue] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState('');
  const toast = useToast();

  const isEditMode = !!variable;

  useEffect(() => {
    if (isOpen) {
        if (variable) {
            setKey(variable.key);
            setValue(variable.value);
        } else {
            setKey('');
            setValue('');
        }
        setError('');
    }
  }, [variable, isOpen]);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!key.trim()) {
      setError('Variable key is required');
      return;
    }
    if (!value.trim()) {
      setError('Variable value is required');
      return;
    }

    try {
      setIsSubmitting(true);
      if (isEditMode && variable) {
        const request: UpdateVariableRequest = {
          id: variable.id!,
          key: key.trim(),
          value: value.trim(),
        };
        const response = await invoke<UpdateVariableResponse>('update_variable', { request });
        if (response.success) {
          toast.success(`Variable "${key.trim()}" updated`);
          onSuccess();
          onClose();
        } else {
          setError(response.message);
        }
      } else {
        const request: CreateVariableRequest = {
          environment_id: environmentId,
          key: key.trim(),
          value: value.trim(),
        };
        const response = await invoke<CreateVariableResponse>('create_variable', { request });
        if (response.success) {
          toast.success(`Variable "${key.trim()}" created`);
          onSuccess();
          onClose();
        } else {
          setError(response.message);
        }
      }
    } catch (err) {
      const action = isEditMode ? 'update' : 'create';
      console.error(`Failed to ${action} variable:`, err);
      setError(err instanceof Error ? err.message : `An unknown error occurred while trying to ${action} the variable.`);
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
    <div className="variable-modal-overlay" onClick={handleOverlayClick}>
      <div className="variable-modal">
        <div className="variable-modal-header">
          <h2>
            <KeyRound size={20} />
            {isEditMode ? 'Edit Variable' : 'Add New Variable'}
          </h2>
          <button 
            className="variable-modal-close"
            onClick={onClose}
            disabled={isSubmitting}
            aria-label="Close modal"
          >
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="variable-modal-body">
          <div className="variable-modal-form-group">
            <label htmlFor="var-key">
              Key <span className="variable-modal-required">*</span>
            </label>
            <input
              id="var-key"
              type="text"
              value={key}
              onChange={(e) => setKey(e.target.value)}
              placeholder="e.g., DATABASE_URL, API_KEY"
              disabled={isSubmitting}
              autoFocus
              required
            />
            <p className="variable-modal-hint">
              Use uppercase with underscores, like: MY_API_KEY
            </p>
          </div>

          <div className="variable-modal-form-group">
            <label htmlFor="var-value">
              Value <span className="variable-modal-required">*</span>
            </label>
            <textarea
              id="var-value"
              value={value}
              onChange={(e) => setValue(e.target.value)}
              placeholder="Enter the value for this variable..."
              rows={4}
              disabled={isSubmitting}
              required
            />
            <div className="variable-modal-hints">
              <p className="variable-modal-hint encrypted">
                <Lock size={12} /> This value will be encrypted at rest.
              </p>
              {key && value && (() => {
                const validation = validateVariable(key, value);
                if (validation.type !== 'none') {
                  return (
                    <p className={`variable-modal-validation ${validation.isValid ? 'valid' : 'invalid'}`}>
                      {validation.isValid ? (
                        <>
                          <CheckCircle2 size={12} />
                          Valid {validation.type}
                        </>
                      ) : (
                        <>
                          <XCircle size={12} />
                          {validation.message}
                        </>
                      )}
                    </p>
                  );
                }
                return null;
              })()}
            </div>
          </div>

          {error && (
            <div className="variable-modal-error-message">
              <AlertTriangle size={16} />
              {error}
            </div>
          )}

          <div className="variable-modal-footer">
            <button
              type="button"
              className="variable-modal-btn-cancel"
              onClick={onClose}
              disabled={isSubmitting}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="variable-modal-btn-submit"
              disabled={isSubmitting || !key.trim() || !value.trim()}
            >
              {isSubmitting ? (isEditMode ? 'Updating...' : 'Adding...') : (isEditMode ? 'Update Variable' : 'Add Variable')}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};