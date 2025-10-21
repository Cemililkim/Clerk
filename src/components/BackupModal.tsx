import React, { useState, useEffect } from 'react';
import { X, Download, Shield, Database, AlertCircle, CheckCircle2 } from 'lucide-react';
import { save } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { createBackup, saveBackupToFile } from '../api/backup';
import { generateBackupFilename, estimateBackupSize } from '../utils/backup';
import './BackupModal.css';

interface BackupModalProps {
  isOpen: boolean;
  onClose: () => void;
}

interface DashboardStats {
  project_count: number;
  environment_count: number;
  variable_count: number;
}

type BackupStatus = 'idle' | 'creating' | 'saving' | 'success' | 'error';

export const BackupModal: React.FC<BackupModalProps> = ({
  isOpen,
  onClose,
}) => {
  const [status, setStatus] = useState<BackupStatus>('idle');
  const [errorMessage, setErrorMessage] = useState<string>('');
  const [backupPath, setBackupPath] = useState<string>('');
  const [progress, setProgress] = useState<string>('');
  const [projectCount, setProjectCount] = useState<number>(0);
  const [environmentCount, setEnvironmentCount] = useState<number>(0);
  const [variableCount, setVariableCount] = useState<number>(0);
  const vaultName = 'vault';

  // Fetch current statistics when modal opens
  useEffect(() => {
    if (isOpen) {
      loadStatistics();
    }
  }, [isOpen]);

  const loadStatistics = async () => {
    try {
      const stats = await invoke<DashboardStats>('get_dashboard_stats');
      setProjectCount(stats.project_count);
      setEnvironmentCount(stats.environment_count);
      setVariableCount(stats.variable_count);
    } catch (error) {
      console.error('Failed to load statistics:', error);
      // Keep zeros as fallback
    }
  };

  const handleCreateBackup = async () => {
    try {
      setStatus('creating');
      setProgress('Creating backup...');
      setErrorMessage('');

      // Create backup
      const backup = await createBackup();

      setProgress('Choosing save location...');
      
      // Generate default filename
      const defaultFilename = generateBackupFilename(vaultName);

      // Open save dialog
      const filePath = await save({
        defaultPath: defaultFilename,
        filters: [{
          name: 'Clerk Backup',
          extensions: ['clerkbackup']
        }],
        title: 'Save Backup File'
      });

      if (!filePath) {
        // User cancelled
        setStatus('idle');
        setProgress('');
        return;
      }

      setStatus('saving');
      setProgress('Saving backup file...');

      // Save to file
      await saveBackupToFile(backup, filePath);

      setStatus('success');
      setBackupPath(filePath);
      setProgress('');

    } catch (error) {
      console.error('Backup creation failed:', error);
      setStatus('error');
      setErrorMessage(error instanceof Error ? error.message : 'Failed to create backup');
      setProgress('');
    }
  };

  const handleClose = () => {
    if (status === 'creating' || status === 'saving') {
      // Don't allow closing during backup
      return;
    }
    setStatus('idle');
    setErrorMessage('');
    setBackupPath('');
    setProgress('');
    onClose();
  };

  const estimatedSize = estimateBackupSize(projectCount, environmentCount, variableCount);

  if (!isOpen) return null;

  return (
    <div className="backup-modal-overlay" onClick={handleClose}>
      <div className="backup-modal" onClick={(e) => e.stopPropagation()}>
        <div className="backup-modal-header">
          <div className="backup-modal-title-section">
            <Shield className="backup-modal-icon" />
            <h2 className="backup-modal-title">Create Vault Backup</h2>
          </div>
          <button 
            className="backup-modal-close" 
            onClick={handleClose}
            disabled={status === 'creating' || status === 'saving'}
          >
            <X size={20} />
          </button>
        </div>

        <div className="backup-modal-body">
          {status === 'idle' && (
            <>
              <div className="backup-modal-info">
                <AlertCircle size={18} />
                <p>
                  This will create an encrypted backup of your vault and all data.
                  You can restore from this backup later if needed.
                </p>
              </div>

              <div className="backup-modal-stats">
                <div className="backup-stat-item">
                  <Database size={16} />
                  <span className="backup-stat-label">Projects:</span>
                  <span className="backup-stat-value">{projectCount}</span>
                </div>
                <div className="backup-stat-item">
                  <Database size={16} />
                  <span className="backup-stat-label">Environments:</span>
                  <span className="backup-stat-value">{environmentCount}</span>
                </div>
                <div className="backup-stat-item">
                  <Database size={16} />
                  <span className="backup-stat-label">Variables:</span>
                  <span className="backup-stat-value">{variableCount}</span>
                </div>
              </div>

              <div className="backup-modal-size-info">
                <span className="backup-size-label">Estimated backup size:</span>
                <span className="backup-size-value">{estimatedSize}</span>
              </div>

              <div className="backup-modal-tips">
                <h4>💡 Backup Tips:</h4>
                <ul>
                  <li>Store backups in a secure location</li>
                  <li>Keep multiple backup copies</li>
                  <li>Test restore periodically</li>
                  <li>Backup file contains encrypted data</li>
                </ul>
              </div>
            </>
          )}

          {(status === 'creating' || status === 'saving') && (
            <div className="backup-modal-progress">
              <div className="backup-progress-spinner">
                <div className="spinner"></div>
              </div>
              <p className="backup-progress-text">{progress}</p>
              <p className="backup-progress-hint">Please wait, do not close this window...</p>
            </div>
          )}

          {status === 'success' && (
            <div className="backup-modal-success">
              <CheckCircle2 size={48} className="backup-success-icon" />
              <h3>Backup Created Successfully!</h3>
              <p className="backup-success-message">
                Your vault has been backed up securely.
              </p>
              <div className="backup-success-path">
                <span className="backup-path-label">Saved to:</span>
                <code className="backup-path-value">{backupPath}</code>
              </div>
              <div className="backup-success-tips">
                <p>✅ Backup file is encrypted</p>
                <p>✅ Store it in a safe location</p>
                <p>✅ You can restore anytime from Settings</p>
              </div>
            </div>
          )}

          {status === 'error' && (
            <div className="backup-modal-error">
              <AlertCircle size={48} className="backup-error-icon" />
              <h3>Backup Failed</h3>
              <p className="backup-error-message">{errorMessage}</p>
              <button 
                className="backup-retry-btn"
                onClick={() => setStatus('idle')}
              >
                Try Again
              </button>
            </div>
          )}
        </div>

        <div className="backup-modal-footer">
          {status === 'idle' && (
            <>
              <button className="backup-modal-btn backup-modal-btn-secondary" onClick={handleClose}>
                Cancel
              </button>
              <button className="backup-modal-btn backup-modal-btn-primary" onClick={handleCreateBackup}>
                <Download size={18} />
                Create Backup
              </button>
            </>
          )}
          {status === 'success' && (
            <button className="backup-modal-btn backup-modal-btn-primary" onClick={handleClose}>
              Done
            </button>
          )}
        </div>
      </div>
    </div>
  );
};
