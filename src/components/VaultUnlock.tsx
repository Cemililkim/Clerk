import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { LockKeyholeOpen, Eye, EyeOff, KeyRound, ShieldCheck } from 'lucide-react';
import './VaultUnlock.css';

interface VaultUnlockProps {
  onVaultUnlocked: () => void;
}

interface UnlockVaultResponse {
  success: boolean;
  message: string;
}

export const VaultUnlock: React.FC<VaultUnlockProps> = ({ onVaultUnlocked }) => {
  const [password, setPassword] = useState('');
  const [rememberMe, setRememberMe] = useState(false);
  const [isUnlocking, setIsUnlocking] = useState(false);
  const [error, setError] = useState('');
  const [showPassword, setShowPassword] = useState(false);

  // Handle form submission
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!password) {
      setError('Please enter your password');
      return;
    }

    try {
      setIsUnlocking(true);
      const response = await invoke<UnlockVaultResponse>('unlock_vault', {
        password,
        rememberMe,
      });

      if (response.success) {
        onVaultUnlocked();
      } else {
        setError(response.message || 'Invalid password');
        setPassword('');
      }
    } catch (err) {
      console.error('Failed to unlock vault:', err);
      setError(err instanceof Error ? err.message : 'Failed to unlock vault');
      setPassword('');
    } finally {
      setIsUnlocking(false);
    }
  };

  return (
    <div className="vault-unlock">
      <div className="vault-unlock-container">
        <div className="vault-unlock-header">
          <div className="vault-unlock-icon">
            <LockKeyholeOpen size={40} strokeWidth={1.5} />
          </div>
          <h1>Welcome Back</h1>
          <p>Enter your master password to unlock your vault.</p>
        </div>

        <form onSubmit={handleSubmit} className="vault-unlock-form">
          <div className="vault-unlock-form-group">
            <div className="vault-unlock-password-input-wrapper">
              <input
                id="password"
                type={showPassword ? 'text' : 'password'}
                value={password}
                onChange={(e) => {
                  setPassword(e.target.value);
                  setError('');
                }}
                placeholder="Master Password"
                disabled={isUnlocking}
                autoFocus
                required
              />
              <button
                type="button"
                className="vault-unlock-toggle-password"
                onClick={() => setShowPassword(!showPassword)}
                disabled={isUnlocking}
                aria-label={showPassword ? 'Hide password' : 'Show password'}
              >
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>
          </div>

          <div className="vault-unlock-checkbox-group">
            <label className="vault-unlock-checkbox-label">
              <input
                type="checkbox"
                checked={rememberMe}
                onChange={(e) => setRememberMe(e.target.checked)}
                disabled={isUnlocking}
              />
              <span>Remember me</span>
            </label>
          </div>

          <div className="vault-unlock-info-box">
            <KeyRound size={16} />
            <p>Uses Windows Credential Manager / macOS Keychain for secure storage</p>
          </div>

          {error && <div className="vault-unlock-error-message">{error}</div>}

          <button
            type="submit"
            className="vault-unlock-button"
            disabled={isUnlocking || !password}
          >
            {isUnlocking ? 'Unlocking...' : 'Unlock Vault'}
          </button>
        </form>

        <div className="vault-unlock-footer">
          <ShieldCheck size={14} />
          <p>Your password is never transmitted or stored</p>
        </div>
      </div>
    </div>
  );
};