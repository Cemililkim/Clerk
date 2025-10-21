import React, { useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Vault, Eye, EyeOff, CheckCircle2, Circle, ShieldCheck } from 'lucide-react';
import './VaultCreation.css';

interface VaultCreationProps {
  onVaultCreated: () => void;
}

interface CreateVaultResponse {
  success: boolean;
  message: string;
}

interface PasswordCriteria {
  length: boolean;
  uppercase: boolean;
  lowercase: boolean;
  number: boolean;
  specialChar: boolean;
}

export const VaultCreation: React.FC<VaultCreationProps> = ({ onVaultCreated }) => {
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState('');
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);
  const [passwordStrength, setPasswordStrength] = useState(0);
  const [acknowledgeRisk, setAcknowledgeRisk] = useState(false);
  const [passwordCriteria, setPasswordCriteria] = useState<PasswordCriteria>({
    length: false,
    uppercase: false,
    lowercase: false,
    number: false,
    specialChar: false,
  });

  const calculatePasswordStrength = (pwd: string): number => {
    let strength = 0;
    if (/[a-z]/.test(pwd)) strength += 15;
    if (/[A-Z]/.test(pwd)) strength += 15;
    if (/[0-9]/.test(pwd)) strength += 15;
    if (/[^a-zA-Z0-9]/.test(pwd)) strength += 15;
    if (pwd.length >= 8) strength += 20;
    if (pwd.length >= 12) strength += 20;
    return Math.min(strength, 100);
  };

  const getStrengthInfo = (strength: number) => {
    if (strength === 0) return { label: '', color: '#6b7280' };
    if (strength < 40) return { label: 'Weak', color: '#ef4444' };
    if (strength < 70) return { label: 'Fair', color: '#f97316' };
    if (strength < 90) return { label: 'Good', color: '#3b82f6' };
    return { label: 'Strong', color: '#22c55e' };
  };

  const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newPassword = e.target.value;
    setPassword(newPassword);
    setPasswordStrength(calculatePasswordStrength(newPassword));
    setPasswordCriteria({
      length: newPassword.length >= 8,
      uppercase: /[A-Z]/.test(newPassword),
      lowercase: /[a-z]/.test(newPassword),
      number: /[0-9]/.test(newPassword),
      specialChar: /[^a-zA-Z0-9]/.test(newPassword),
    });
    setError('');
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    if (!isFormValid) {
        setError('Please ensure all password criteria are met and passwords match.');
        return;
    }
    try {
      setIsCreating(true);
      const response = await invoke<CreateVaultResponse>('create_vault', { password });
      if (response.success) {
        onVaultCreated();
      } else {
        setError(response.message || 'Failed to create vault');
      }
    } catch (err) {
      console.error('Failed to create vault:', err);
      setError(err instanceof Error ? err.message : 'Failed to create vault');
    } finally {
      setIsCreating(false);
    }
  };

  const passwordsMatch = useMemo(() => {
    if (!confirmPassword) return true; // Don't show error if confirm password is empty
    return password === confirmPassword;
  }, [password, confirmPassword]);

  const isFormValid = useMemo(() => {
    const allCriteriaMet = Object.values(passwordCriteria).every(Boolean);
    const passwordsMatch = password && password === confirmPassword;
    return allCriteriaMet && passwordsMatch && acknowledgeRisk;
  }, [password, confirmPassword, passwordCriteria, acknowledgeRisk]);

  const strengthInfo = getStrengthInfo(passwordStrength);

  const CriteriaItem: React.FC<{ label: string; met: boolean }> = ({ label, met }) => (
    <li className={`criteria-item ${met ? 'valid' : 'invalid'}`}>
      {met ? <CheckCircle2 size={16} /> : <Circle size={16} />}
      <span>{label}</span>
    </li>
  );

  return (
    <div className="vault-creation">
      <div className="vault-creation-container">
        <div className="vault-creation-header">
          <div className="vault-creation-icon">
            <Vault size={40} strokeWidth={1.5} />
          </div>
          <h1>Create Your Vault</h1>
          <p>Choose a strong password to protect your data.</p>
        </div>

        <form onSubmit={handleSubmit} className="vault-creation-form">
          <div className="vault-creation-form-group">
            <div className="vault-creation-password-input-wrapper">
              <input
                id="password"
                type={showPassword ? 'text' : 'password'}
                value={password}
                onChange={handlePasswordChange}
                placeholder="Master Password"
                disabled={isCreating}
                autoFocus
                required
              />
              <button
                type="button"
                className="vault-creation-toggle-password"
                onClick={() => setShowPassword(!showPassword)}
                disabled={isCreating}
                aria-label="Toggle password visibility"
              >
                {showPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>
            {password && (
              <div className="vault-creation-password-strength">
                <div className="vault-creation-strength-bar">
                  <div
                    className="vault-creation-strength-fill"
                    style={{
                      width: `${passwordStrength}%`,
                      backgroundColor: strengthInfo.color,
                    }}
                  />
                </div>
                <span className="vault-creation-strength-label" style={{ color: strengthInfo.color }}>
                  {strengthInfo.label}
                </span>
              </div>
            )}
          </div>
          
          <div className="vault-creation-form-group">
            <div className="vault-creation-password-input-wrapper">
              <input
                id="confirm-password"
                type={showConfirmPassword ? 'text' : 'password'}
                value={confirmPassword}
                onChange={(e) => {
                  setConfirmPassword(e.target.value);
                  setError('');
                }}
                placeholder="Confirm Password"
                disabled={isCreating}
                required
              />
              <button
                type="button"
                className="vault-creation-toggle-password"
                onClick={() => setShowConfirmPassword(!showConfirmPassword)}
                disabled={isCreating}
                aria-label="Toggle confirm password visibility"
              >
                {showConfirmPassword ? <EyeOff size={18} /> : <Eye size={18} />}
              </button>
            </div>
            {confirmPassword && !passwordsMatch && (
              <div className="vault-creation-password-mismatch">
                Passwords do not match
              </div>
            )}
          </div>

          <div className="vault-creation-criteria">
            <ul>
              <CriteriaItem label="At least 8 characters" met={passwordCriteria.length} />
              <CriteriaItem label="An uppercase letter (A-Z)" met={passwordCriteria.uppercase} />
              <CriteriaItem label="A lowercase letter (a-z)" met={passwordCriteria.lowercase} />
              <CriteriaItem label="A number (0-9)" met={passwordCriteria.number} />
              <CriteriaItem label="A special character (!@#...)" met={passwordCriteria.specialChar} />
            </ul>
          </div>
          
          {error && <div className="vault-creation-error-message">{error}</div>}

          <div className="vault-creation-footer">
            <div className="vault-creation-warning-box">
              <ShieldCheck size={16} />
              <p>
                <strong>Lost passwords cannot be recovered.</strong>
                {' '}If you lose your password, all your data may be lost!
              </p>
            </div>
            <label className="vault-creation-acknowledge">
              <input
                type="checkbox"
                checked={acknowledgeRisk}
                onChange={(e) => setAcknowledgeRisk(e.target.checked)}
                disabled={isCreating}
              />
              <span>I understand and accept the risk</span>
            </label>
          </div>

          <button
            type="submit"
            className="vault-creation-button"
            disabled={!isFormValid || isCreating}
          >
            {isCreating ? 'Creating...' : 'Create Vault'}
          </button>
        </form>
      </div>
    </div>
  );
};