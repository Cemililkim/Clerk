import React, { useState } from 'react';
import { X, Upload, AlertTriangle, CheckCircle2, Database, Calendar, Package } from 'lucide-react';
import { open } from '@tauri-apps/plugin-dialog';
import { restoreBackup, getBackupInfo, validateBackupFile, loadBackupFromFile } from '../api/backup';
import { formatBackupDate } from '../utils/backup';
import type { BackupMetadata } from '../api/backup';
import './RestoreModal.css';

interface RestoreModalProps {
  isOpen: boolean;
  onClose: () => void;
  onRestoreSuccess: () => void;
}

type RestoreStatus = 'idle' | 'selecting' | 'validating' | 'preview' | 'restoring' | 'success' | 'error';

export const RestoreModal: React.FC<RestoreModalProps> = ({
  isOpen,
  onClose,
  onRestoreSuccess,
}) => {
  const [status, setStatus] = useState<RestoreStatus>('idle');
  const [errorMessage, setErrorMessage] = useState<string>('');
  const [backupData, setBackupData] = useState<string>('');
  const [backupMetadata, setBackupMetadata] = useState<BackupMetadata | null>(null);
  const [backupFilePath, setBackupFilePath] = useState<string>('');
  const [progress, setProgress] = useState<string>('');

  const handleSelectBackup = async () => {
    try {
      setStatus('selecting');
      setErrorMessage('');

      // Open file dialog
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'Clerk Backup',
          extensions: ['clerkbackup']
        }],
        title: 'Select Backup File'
      });

      if (!selected) {
        // User cancelled
        setStatus('idle');
        return;
      }

      setBackupFilePath(selected as string);
      setStatus('validating');
      setProgress('Loading backup file...');

      // Load backup file
      const backup = await loadBackupFromFile(selected as string);
      const backupJson = JSON.stringify(backup);

      // Validate backup
      const isValid = await validateBackupFile(backupJson);
      
      if (!isValid) {
        throw new Error('Invalid backup file format');
      }

      // Get backup info
      const metadata = await getBackupInfo(backupJson);
      
      setBackupData(backupJson);
      setBackupMetadata(metadata);
      setStatus('preview');
      setProgress('');

    } catch (error) {
      console.error('Failed to load backup:', error);
      setStatus('error');
      setErrorMessage(error instanceof Error ? error.message : 'Failed to load backup file');
      setProgress('');
    }
  };

  const handleRestore = async () => {
    if (!backupData || !backupMetadata) {
      return;
    }

    try {
      setStatus('restoring');
      setProgress('Restoring backup...');
      setErrorMessage('');

      // Restore backup
      await restoreBackup(backupData);

      setStatus('success');
      setProgress('');

      // Trigger reload/refresh
      onRestoreSuccess();

    } catch (error) {
      console.error('Restore failed:', error);
      setStatus('error');
      setErrorMessage(error instanceof Error ? error.message : 'Failed to restore backup');
      setProgress('');
    }
  };

  const handleClose = () => {
    if (status === 'restoring') {
      // Don't allow closing during restore
      return;
    }

    if (status === 'success') {
      // Trigger reload
      onRestoreSuccess();
    }

    setStatus('idle');
    setErrorMessage('');
    setBackupData('');
    setBackupMetadata(null);
    setBackupFilePath('');
    setProgress('');
    onClose();
  };

  const handleTryAgain = () => {
    setStatus('idle');
    setErrorMessage('');
    setBackupData('');
    setBackupMetadata(null);
    setBackupFilePath('');
  };

  if (!isOpen) return null;

  return (
    <div className="restore-modal-overlay" onClick={handleClose}>
      <div className="restore-modal" onClick={(e) => e.stopPropagation()}>
        <div className="restore-modal-header">
          <div className="restore-modal-title-section">
            <Upload className="restore-modal-icon" />
            <h2 className="restore-modal-title">Restore from Backup</h2>
          </div>
          <button 
            className="restore-modal-close" 
            onClick={handleClose}
            disabled={status === 'restoring'}
          >
            <X size={20} />
          </button>
        </div>

        <div className="restore-modal-body">
          {status === 'idle' && (
            <>
              <div className="restore-modal-warning">
                <AlertTriangle size={18} />
                <div>
                  <p><strong>Warning:</strong> Restoring will replace your current vault data.</p>
                  <p>Your existing vault and database will be backed up automatically before restore.</p>
                </div>
              </div>

              <div className="restore-modal-actions">
                <button className="restore-select-btn" onClick={handleSelectBackup}>
                  <Upload size={20} />
                  Select Backup File
                </button>
              </div>

              <div className="restore-modal-tips">
                <h4>📋 Before You Restore:</h4>
                <ul>
                  <li>Ensure the backup file is from a trusted source</li>
                  <li>Verify the backup contains the data you expect</li>
                  <li>Close all applications using the vault</li>
                  <li>Your current data will be preserved as .backup files</li>
                </ul>
              </div>
            </>
          )}

          {(status === 'selecting' || status === 'validating') && (
            <div className="restore-modal-progress">
              <div className="restore-progress-spinner">
                <div className="spinner"></div>
              </div>
              <p className="restore-progress-text">{progress || 'Loading...'}</p>
            </div>
          )}

          {status === 'preview' && backupMetadata && (
            <>
              <div className="restore-preview-header">
                <h3>Backup Preview</h3>
                <span className="restore-preview-badge">Ready to Restore</span>
              </div>

              <div className="restore-preview-info">
                <div className="restore-info-item">
                  <Package size={16} />
                  <span className="restore-info-label">Vault Name:</span>
                  <span className="restore-info-value">{backupMetadata.vaultName}</span>
                </div>
                <div className="restore-info-item">
                  <Calendar size={16} />
                  <span className="restore-info-label">Created:</span>
                  <span className="restore-info-value">{formatBackupDate(backupMetadata.createdAt)}</span>
                </div>
                <div className="restore-info-item">
                  <Database size={16} />
                  <span className="restore-info-label">Version:</span>
                  <span className="restore-info-value">{backupMetadata.version}</span>
                </div>
              </div>

              <div className="restore-stats">
                <div className="restore-stat-card">
                  <div className="restore-stat-value">{backupMetadata.projectCount}</div>
                  <div className="restore-stat-label">Projects</div>
                </div>
                <div className="restore-stat-card">
                  <div className="restore-stat-value">{backupMetadata.environmentCount}</div>
                  <div className="restore-stat-label">Environments</div>
                </div>
                <div className="restore-stat-card">
                  <div className="restore-stat-value">{backupMetadata.variableCount}</div>
                  <div className="restore-stat-label">Variables</div>
                </div>
              </div>

              <div className="restore-file-path">
                <span className="restore-path-label">Backup File:</span>
                <code className="restore-path-value">{backupFilePath}</code>
              </div>

              <div className="restore-modal-warning">
                <AlertTriangle size={18} />
                <p>
                  This will replace your current vault with the backup data.
                  Are you sure you want to continue?
                </p>
              </div>
            </>
          )}

          {status === 'restoring' && (
            <div className="restore-modal-progress">
              <div className="restore-progress-spinner">
                <div className="spinner"></div>
              </div>
              <p className="restore-progress-text">{progress}</p>
              <p className="restore-progress-hint">Please wait, do not close this window...</p>
            </div>
          )}

          {status === 'success' && (
            <div className="restore-modal-success">
              <CheckCircle2 size={48} className="restore-success-icon" />
              <h3>Restore Completed!</h3>
              <p className="restore-success-message">
                Your vault has been successfully restored from the backup.
              </p>
              <div className="restore-success-tips">
                <p>✅ Vault data restored</p>
                <p>✅ Database restored</p>
                <p>✅ Previous data saved as .backup files</p>
                <p>💡 The application will reload to apply changes</p>
              </div>
            </div>
          )}

          {status === 'error' && (
            <div className="restore-modal-error">
              <AlertTriangle size={48} className="restore-error-icon" />
              <h3>Restore Failed</h3>
              <p className="restore-error-message">{errorMessage}</p>
              <button 
                className="restore-retry-btn"
                onClick={handleTryAgain}
              >
                Try Again
              </button>
            </div>
          )}
        </div>

        <div className="restore-modal-footer">
          {(status === 'idle' || status === 'error') && (
            <button className="restore-modal-btn restore-modal-btn-secondary" onClick={handleClose}>
              Cancel
            </button>
          )}
          {status === 'preview' && (
            <>
              <button className="restore-modal-btn restore-modal-btn-secondary" onClick={handleTryAgain}>
                Choose Different Backup
              </button>
              <button className="restore-modal-btn restore-modal-btn-primary" onClick={handleRestore}>
                <Upload size={18} />
                Restore Now
              </button>
            </>
          )}
          {status === 'success' && (
            <button className="restore-modal-btn restore-modal-btn-primary" onClick={handleClose}>
              Done
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
