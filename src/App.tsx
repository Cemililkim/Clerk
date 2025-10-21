import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { VaultCreation } from './components/VaultCreation';
import { VaultUnlock } from './components/VaultUnlock';
import { VaultDashboard } from './components/VaultDashboard';
import { ToastProvider } from './components/Toast';
import { ThemeProvider } from './contexts/ThemeContext';
import { Loader } from 'lucide-react';
import './styles/App.css';
import './styles/dark-mode.css';

type AppState = 'loading' | 'auto-unlocking' | 'create' | 'unlock' | 'main';

interface UnlockVaultResponse {
  success: boolean;
}

function App(): React.ReactElement {
  const [appState, setAppState] = useState<AppState>('loading');

  useEffect(() => {
    checkVaultStatus();
  }, []);

  const checkVaultStatus = async () => {
    try {
      const vaultExists = await invoke<boolean>('check_vault_exists');
      
      if (!vaultExists) {
        setAppState('create');
        return;
      }

      setAppState('auto-unlocking');
      
      try {
        const response = await invoke<UnlockVaultResponse>('auto_unlock');
        if (response.success) {
          console.log('Auto-unlock successful');
          setAppState('main');
        } else {
          setAppState('unlock');
        }
      } catch (error) {
        console.log('Auto-unlock not available:', error);
        setAppState('unlock');
      }
    } catch (error) {
      console.error('Failed to check vault status:', error);
      setAppState('create');
    }
  };

  const handleVaultCreated = () => {
    setAppState('main');
  };

  const handleVaultUnlocked = () => {
    setAppState('main');
  };

  const handleLock = () => {
    setAppState('unlock');
  };

  return (
    <ThemeProvider>
      <ToastProvider>
        {(appState === 'loading' || appState === 'auto-unlocking') && (
          <div className="app-loading-container">
            <Loader size={48} className="app-loading-spinner" />
            <p className="app-loading-text">
              {appState === 'loading' ? 'Checking vault status...' : 'Unlocking vault...'}
            </p>
          </div>
        )}

        {appState === 'create' && <VaultCreation onVaultCreated={handleVaultCreated} />}

        {appState === 'unlock' && <VaultUnlock onVaultUnlocked={handleVaultUnlocked} />}

        {appState === 'main' && <VaultDashboard onLock={handleLock} />}
      </ToastProvider>
    </ThemeProvider>
  );
}

export default App;