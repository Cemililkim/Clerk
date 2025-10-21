import React, { useState } from 'react';
import { AlertTriangle } from 'lucide-react';
import './DeleteConfirmModal.css';

interface DeleteConfirmModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => Promise<void>;
  title: string;
  message: string;
  itemName: string;
}

export const DeleteConfirmModal: React.FC<DeleteConfirmModalProps> = ({
  isOpen,
  onClose,
  onConfirm,
  title,
  message,
  itemName,
}) => {
  const [isDeleting, setIsDeleting] = useState(false);
  const [error, setError] = useState('');

  if (!isOpen) return null;

  const handleConfirm = async () => {
    try {
      setIsDeleting(true);
      setError('');
      await onConfirm();
      onClose();
    } catch (err) {
      console.error('Failed to delete:', err);
      setError(err instanceof Error ? err.message : 'An unknown error occurred.');
    } finally {
      setIsDeleting(false);
    }
  };

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget && !isDeleting) {
      onClose();
    }
  };

  return (
    <div className="delete-confirm-modal-overlay" onClick={handleOverlayClick}>
      <div className="delete-confirm-modal-content">
        <div className="delete-confirm-modal-icon-wrapper">
          <AlertTriangle size={48} className="delete-confirm-modal-icon" />
        </div>

        <div className="delete-confirm-modal-body">
          <h2>{title}</h2>
          <p>{message}</p>
          
          {itemName && (
            <div className="delete-confirm-modal-item-name">
              <code>{itemName}</code>
            </div>
          )}

          <p className="delete-confirm-modal-warning">
            This action cannot be undone.
          </p>

          {error && (
            <div className="delete-confirm-modal-error-message">
              <AlertTriangle size={16} />
              {error}
            </div>
          )}
        </div>

        <div className="delete-confirm-modal-footer">
          <button
            type="button"
            className="delete-confirm-modal-btn-cancel"
            onClick={onClose}
            disabled={isDeleting}
          >
            Cancel
          </button>
          <button
            type="button"
            className="delete-confirm-modal-btn-confirm"
            onClick={handleConfirm}
            disabled={isDeleting}
          >
            {isDeleting ? 'Deleting...' : 'Yes, Delete'}
          </button>
        </div>
      </div>
    </div>
  );
};