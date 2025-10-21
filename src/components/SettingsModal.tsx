import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from './Toast';
import { useTheme, ThemeColor } from '../contexts/ThemeContext';
import { Settings, X, Lock, Info, Moon, Sun, Palette, Keyboard, Shield, Download, Upload, Terminal, CheckCircle, AlertCircle, BookOpen, ChevronDown, ChevronUp } from 'lucide-react';
import { BackupModal } from './BackupModal';
import { RestoreModal } from './RestoreModal';
import { UpdateChecker } from './UpdateChecker';
import './SettingsModal.css';

interface SettingsModalProps {
  isOpen: boolean;
  onClose: () => void;
  onRestoreSuccess?: () => void;
}

const TIMEOUT_OPTIONS = [
  { value: 0, label: 'Disabled' },
  { value: 5, label: '5 min' },
  { value: 10, label: '10 min' },
  { value: 15, label: '15 min' },
  { value: 30, label: '30 min' },
  { value: 60, label: '1 hour' },
];

export const SettingsModal: React.FC<SettingsModalProps> = ({ 
  isOpen, 
  onClose,
  onRestoreSuccess,
}) => {
  const [lockTimeout, setLockTimeout] = useState(0);
  const [isSaving, setIsSaving] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [isBackupModalOpen, setIsBackupModalOpen] = useState(false);
  const [isRestoreModalOpen, setIsRestoreModalOpen] = useState(false);
  const [isPathAdded, setIsPathAdded] = useState(false);
  const [isCheckingPath, setIsCheckingPath] = useState(false);
  const [isAddingPath, setIsAddingPath] = useState(false);
  const [isCliGuideExpanded, setIsCliGuideExpanded] = useState(false);
  const toast = useToast();
  const { isDarkMode, themeColor, toggleDarkMode, setThemeColor } = useTheme();

  const THEME_COLORS: Array<{ value: ThemeColor; label: string; color: string }> = [
    { value: 'purple', label: 'Purple', color: '#9333ea' },
    { value: 'blue', label: 'Blue', color: '#2563eb' },
    { value: 'green', label: 'Green', color: '#059669' },
    { value: 'orange', label: 'Orange', color: '#ea580c' },
    { value: 'pink', label: 'Pink', color: '#db2777' },
  ];

  useEffect(() => {
    if (isOpen) {
      loadSettings();
      checkPathStatus();
    }
  }, [isOpen]);

  const checkPathStatus = async () => {
    try {
      setIsCheckingPath(true);
      const isInPath = await invoke<boolean>('check_cli_in_path');
      setIsPathAdded(isInPath);
    } catch (err) {
      console.error('Failed to check PATH status:', err);
    } finally {
      setIsCheckingPath(false);
    }
  };

  const handleAddToPath = async () => {
    try {
      setIsAddingPath(true);
      await invoke('add_cli_to_path');
      setIsPathAdded(true);
      toast.success('Clerk CLI added to PATH! Please restart your terminals.');
    } catch (err) {
      console.error('Failed to add to PATH:', err);
      const errorMessage = err instanceof Error ? err.message : 'Failed to add to PATH';
      toast.error(errorMessage);
    } finally {
      setIsAddingPath(false);
    }
  };

  const handleRemoveFromPath = async () => {
    try {
      setIsAddingPath(true);
      await invoke('remove_cli_from_path');
      setIsPathAdded(false);
      toast.success('Clerk CLI removed from PATH');
    } catch (err) {
      console.error('Failed to remove from PATH:', err);
      const errorMessage = err instanceof Error ? err.message : 'Failed to remove from PATH';
      toast.error(errorMessage);
    } finally {
      setIsAddingPath(false);
    }
  };

  const loadSettings = async () => {
    try {
      setIsLoading(true);
      const timeout = await invoke<number>('get_lock_timeout');
      setLockTimeout(timeout);
    } catch (err) {
      console.error('Failed to load settings:', err);
      toast.error('Failed to load settings');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setIsSaving(true);
      await invoke('set_lock_timeout', { timeoutMinutes: lockTimeout });
      toast.success('Settings saved successfully');
      setTimeout(() => {
        onClose();
      }, 500);
    } catch (err) {
      console.error('Failed to save settings:', err);
      const errorMessage = err instanceof Error ? err.message : 'Failed to save settings';
      toast.error(errorMessage);
    } finally {
      setIsSaving(false);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="settings-modal-overlay" onClick={onClose}>
      <div className="settings-modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="settings-modal-header">
          <h2><Settings size={20} /> Settings</h2>
          <button className="settings-modal-close-button" onClick={onClose}>
            <X size={20} />
          </button>
        </div>

        <div className="settings-modal-body">
          {isLoading ? (
            <div className="settings-modal-loading-state">Loading settings...</div>
          ) : (
            <>
              <div className="settings-modal-section">
                <h3>{isDarkMode ? <Moon size={16} /> : <Sun size={16} />} Appearance</h3>
                
                <div className="settings-modal-item">
                  <label>Theme Mode</label>
                  <p className="settings-modal-description">
                    Choose between light and dark mode.
                  </p>
                  <button
                    type="button"
                    className="settings-modal-theme-toggle"
                    onClick={toggleDarkMode}
                  >
                    {isDarkMode ? (
                      <>
                        <Moon size={18} />
                        <span>Dark Mode</span>
                      </>
                    ) : (
                      <>
                        <Sun size={18} />
                        <span>Light Mode</span>
                      </>
                    )}
                  </button>
                </div>

                <div className="settings-modal-item">
                  <label><Palette size={16} /> Color Theme</label>
                  <p className="settings-modal-description">
                    Choose your preferred accent color.
                  </p>
                  <div className="settings-modal-color-grid">
                    {THEME_COLORS.map((theme) => (
                      <button
                        key={theme.value}
                        type="button"
                        className={`settings-modal-color-button ${themeColor === theme.value ? 'active' : ''}`}
                        onClick={() => setThemeColor(theme.value)}
                        title={theme.label}
                      >
                        <div 
                          className="settings-modal-color-swatch" 
                          style={{ background: theme.color }}
                        />
                        <span>{theme.label}</span>
                      </button>
                    ))}
                  </div>
                </div>
              </div>

              <div className="settings-modal-section">
                <h3><Lock size={16} /> Security</h3>
                <div className="settings-modal-item">
                <label>Auto-lock timeout</label>
                <p className="settings-modal-description">
                  Automatically lock the vault after a period of inactivity.
                </p>
                {/* --- DEĞİŞEN KISIM: Segmented Control / Button Group --- */}
                <div className="settings-modal-button-group">
                  {TIMEOUT_OPTIONS.map((option) => (
                    <button
                      key={option.value}
                      type="button"
                      className={`settings-modal-option-button ${lockTimeout === option.value ? 'active' : ''}`}
                      onClick={() => setLockTimeout(option.value)}
                      disabled={isSaving}
                    >
                      {option.label}
                    </button>
                  ))}
                </div>
              </div>

              {lockTimeout > 0 && (
                <div className="settings-modal-info-box">
                  <Info size={16} />
                  <p>The vault will lock after {TIMEOUT_OPTIONS.find(o => o.value === lockTimeout)?.label} of inactivity (mouse movement, clicks, or typing).</p>
                </div>
              )}
              </div>

              <div className="settings-modal-section">
                <h3><Terminal size={16} /> CLI Integration</h3>
                <p className="settings-modal-description">
                  Add Clerk CLI to your system PATH to use 'clerk' commands from any terminal.
                </p>

                <div className="settings-modal-path-status">
                  {isCheckingPath ? (
                    <div className="settings-modal-path-checking">Checking PATH status...</div>
                  ) : isPathAdded ? (
                    <div className="settings-modal-path-added">
                      <CheckCircle size={18} />
                      <div>
                        <div className="settings-modal-path-status-title">CLI is in PATH</div>
                        <div className="settings-modal-path-status-subtitle">
                          You can use 'clerk' command from any terminal
                        </div>
                      </div>
                    </div>
                  ) : (
                    <div className="settings-modal-path-not-added">
                      <AlertCircle size={18} />
                      <div>
                        <div className="settings-modal-path-status-title">CLI is not in PATH</div>
                        <div className="settings-modal-path-status-subtitle">
                          Add to PATH to use 'clerk' commands globally
                        </div>
                      </div>
                    </div>
                  )}
                </div>

                <div className="settings-modal-path-actions">
                  {isPathAdded ? (
                    <button
                      type="button"
                      className="settings-modal-path-btn remove"
                      onClick={handleRemoveFromPath}
                      disabled={isAddingPath}
                    >
                      {isAddingPath ? 'Removing...' : 'Remove from PATH'}
                    </button>
                  ) : (
                    <button
                      type="button"
                      className="settings-modal-path-btn add"
                      onClick={handleAddToPath}
                      disabled={isAddingPath}
                    >
                      <Terminal size={18} />
                      {isAddingPath ? 'Adding to PATH...' : 'Add to PATH'}
                    </button>
                  )}
                </div>

                <div className="settings-modal-info-box">
                  <Info size={16} />
                  <p>
                    After adding to PATH, restart your terminal and try: <code>clerk --version</code>
                  </p>
                </div>

                {/* CLI Quick Start Guide */}
                <div className="settings-modal-cli-guide">
                  <button
                    type="button"
                    className="settings-modal-cli-guide-toggle"
                    onClick={() => setIsCliGuideExpanded(!isCliGuideExpanded)}
                  >
                    <BookOpen size={18} />
                    <span>CLI Quick Start Guide</span>
                    {isCliGuideExpanded ? <ChevronUp size={18} /> : <ChevronDown size={18} />}
                  </button>

                  {isCliGuideExpanded && (
                    <div className="settings-modal-cli-guide-content">
                      <div className="settings-modal-cli-guide-section">
                        <h4>🚀 Basic Commands</h4>
                        <div className="settings-modal-cli-command">
                          <code>clerk unlock</code>
                          <span>Unlock your vault (creates a session)</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk get API_KEY -p my-app -e prod</code>
                          <span>Get a variable value</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk set DB_URL postgres://... -p my-app -e dev</code>
                          <span>Set a variable</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk ls -p my-app -e prod --show-values</code>
                          <span>List all variables (alias: list)</span>
                        </div>
                      </div>

                      <div className="settings-modal-cli-guide-section">
                        <h4>📦 Project & Environment</h4>
                        <div className="settings-modal-cli-command">
                          <code>clerk pc my-app -d "My application"</code>
                          <span>Create project (alias: project-create)</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk pl</code>
                          <span>List all projects (alias: project-list)</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk ec staging -p my-app</code>
                          <span>Create environment (alias: env-create)</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk el -p my-app</code>
                          <span>List environments (alias: env-list)</span>
                        </div>
                      </div>

                      <div className="settings-modal-cli-guide-section">
                        <h4>🔄 Bulk Operations</h4>
                        <div className="settings-modal-cli-command">
                          <code>clerk imp .env.local -p my-app -e dev --overwrite</code>
                          <span>Import from .env file (alias: import)</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk export -p my-app -e prod -o .env</code>
                          <span>Export to .env file</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk cp API_KEY --from-project app1 --from-env dev --to-project app2 --to-env prod</code>
                          <span>Copy variable between environments</span>
                        </div>
                      </div>

                      <div className="settings-modal-cli-guide-section">
                        <h4>🔐 Session Management</h4>
                        <div className="settings-modal-cli-command">
                          <code>clerk status</code>
                          <span>Check if vault is unlocked</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk lock</code>
                          <span>Lock vault and clear session</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk -S get API_KEY -p app -e prod</code>
                          <span>Skip session (always prompt for password)</span>
                        </div>
                      </div>

                      <div className="settings-modal-cli-guide-section">
                        <h4>⚡ Advanced Usage</h4>
                        <div className="settings-modal-cli-command">
                          <code>clerk run -p my-app -e prod -- npm start</code>
                          <span>Run command with injected env variables</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk d API_KEY -p my-app -e dev --force</code>
                          <span>Delete variable (alias: delete)</span>
                        </div>
                        <div className="settings-modal-cli-command">
                          <code>clerk --help</code>
                          <span>View all available commands</span>
                        </div>
                      </div>

                      <div className="settings-modal-cli-guide-tips">
                        <strong>💡 Pro Tips:</strong>
                        <ul>
                          <li>Session persists across commands in the same terminal</li>
                          <li>Use short aliases for faster typing: <code>g</code>, <code>s</code>, <code>ls</code>, <code>d</code>, <code>cp</code></li>
                          <li>Use <code>--force</code> flag to skip confirmation prompts</li>
                          <li>Comments in .env files are ignored during import</li>
                          <li>Each terminal session has isolated password cache</li>
                        </ul>
                      </div>
                    </div>
                  )}
                </div>
              </div>

              <div className="settings-modal-section">
                <h3><Shield size={16} /> Backup & Restore</h3>
                <p className="settings-modal-description">
                  Create encrypted backups of your vault and restore from previous backups.
                </p>

                <div className="settings-modal-backup-actions">
                  <button
                    type="button"
                    className="settings-modal-backup-btn"
                    onClick={() => setIsBackupModalOpen(true)}
                  >
                    <Download size={18} />
                    <div>
                      <div className="settings-modal-backup-btn-title">Create Backup</div>
                      <div className="settings-modal-backup-btn-subtitle">
                        Export all data securely
                      </div>
                    </div>
                  </button>

                  <button
                    type="button"
                    className="settings-modal-backup-btn"
                    onClick={() => setIsRestoreModalOpen(true)}
                  >
                    <Upload size={18} />
                    <div>
                      <div className="settings-modal-backup-btn-title">Restore Backup</div>
                      <div className="settings-modal-backup-btn-subtitle">
                        Import from backup file
                      </div>
                    </div>
                  </button>
                </div>

                <div className="settings-modal-info-box">
                  <Info size={16} />
                  <p>Backups include all projects, environments, and variables in an encrypted format.</p>
                </div>
              </div>

              {/* Update Checker */}
              <div className="settings-modal-section">
                <UpdateChecker />
              </div>

              <div className="settings-modal-section">
                <h3><Keyboard size={16} /> Keyboard Shortcuts</h3>
                <div className="settings-modal-shortcuts">
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>N</kbd>
                    <span>New Project</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>E</kbd>
                    <span>New Environment</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>V</kbd>
                    <span>New Variable</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>F</kbd>
                    <span>Focus Search</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>R</kbd>
                    <span>Refresh Projects</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>A</kbd>
                    <span>Audit Log</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>,</kbd>
                    <span>Settings</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Ctrl</kbd> + <kbd>Shift</kbd> + <kbd>L</kbd>
                    <span>Lock Vault</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Esc</kbd>
                    <span>Close Modal / Clear Search</span>
                  </div>
                  <div className="settings-modal-shortcut">
                    <kbd>Shift</kbd> + <kbd>Click</kbd>
                    <span>Delete without confirmation</span>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>

        <div className="settings-modal-footer">
          <button
            type="button"
            onClick={onClose}
            disabled={isSaving}
            className="settings-modal-btn-secondary"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={handleSave}
            disabled={isSaving || isLoading}
            className="settings-modal-btn-primary"
          >
            {isSaving ? 'Saving...' : 'Save Settings'}
          </button>
        </div>
      </div>

      <BackupModal
        isOpen={isBackupModalOpen}
        onClose={() => setIsBackupModalOpen(false)}
      />

      <RestoreModal
        isOpen={isRestoreModalOpen}
        onClose={() => setIsRestoreModalOpen(false)}
        onRestoreSuccess={() => {
          setIsRestoreModalOpen(false);
          if (onRestoreSuccess) {
            onRestoreSuccess();
          }
        }}
      />
    </div>
  );
};