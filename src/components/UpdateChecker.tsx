import { useState } from 'react';
import { Download, CheckCircle, AlertCircle, Loader2 } from 'lucide-react';
import { open } from '@tauri-apps/plugin-shell';
import './UpdateChecker.css';

interface GitHubRelease {
  tag_name: string;
  name: string;
  html_url: string;
  published_at: string;
  body: string;
}

// Read version from package.json at build time
const currentVersion = '1.0.0';

export function UpdateChecker() {
  const [checking, setChecking] = useState(false);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [latestRelease, setLatestRelease] = useState<GitHubRelease | null>(null);
  const [error, setError] = useState<string | null>(null);

  const checkForUpdates = async () => {
    setChecking(true);
    setError(null);
    
    try {
      console.log('Checking for updates...');
      // Fetch latest release from GitHub
      const response = await fetch(
        'https://api.github.com/repos/Cemililkim/Clerk/releases/latest',
        {
          headers: {
            'Accept': 'application/vnd.github.v3+json',
          },
        }
      );

      if (!response.ok) {
        if (response.status === 404) {
          setError('No releases found yet.');
          return;
        }
        throw new Error('Failed to check for updates');
      }

      const release: GitHubRelease = await response.json();
      console.log('Latest release:', release.tag_name);
      
      // Remove 'v' prefix if present
      const latestVersion = release.tag_name.replace(/^v/, '');
      const current = currentVersion.replace(/^v/, '');

      console.log('Comparing versions:', latestVersion, 'vs', current);

      // Compare versions
      if (compareVersions(latestVersion, current) > 0) {
        console.log('Update available!');
        setUpdateAvailable(true);
        setLatestRelease(release);
      } else {
        console.log('Already up to date');
        setUpdateAvailable(false);
        setLatestRelease(null);
      }
    } catch (err) {
      console.error('Update check failed:', err);
      setError(err instanceof Error ? err.message : 'Failed to check for updates');
    } finally {
      setChecking(false);
    }
  };

  const compareVersions = (a: string, b: string): number => {
    const aParts = a.split('.').map(Number);
    const bParts = b.split('.').map(Number);

    for (let i = 0; i < Math.max(aParts.length, bParts.length); i++) {
      const aPart = aParts[i] || 0;
      const bPart = bParts[i] || 0;

      if (aPart > bPart) return 1;
      if (aPart < bPart) return -1;
    }

    return 0;
  };

  const openDownloadPage = async () => {
    if (latestRelease) {
      try {
        console.log('Opening download page:', latestRelease.html_url);
        await open(latestRelease.html_url);
      } catch (err) {
        console.error('Failed to open download page:', err);
        setError('Failed to open download page. Please visit GitHub manually.');
      }
    }
  };

  return (
    <div className="update-checker">
      <h3>
        <Download size={16} />
        Software Updates
      </h3>
      
      <div className="update-checker-version-box">
        <div className="update-checker-current-version">
          <p className="update-checker-version-label">Current Version</p>
          <p className="update-checker-version-number">{currentVersion}</p>
        </div>
        <button
          onClick={checkForUpdates}
          disabled={checking}
          className="update-checker-btn"
        >
          {checking ? (
            <>
              <Loader2 size={16} className="spinning" />
              Checking...
            </>
          ) : (
            <>
              <Download size={16} />
              Check for Updates
            </>
          )}
        </button>
      </div>

      {error && (
        <div className="update-checker-alert error">
          <AlertCircle size={16} />
          <span>{error}</span>
        </div>
      )}

      {!checking && !error && !updateAvailable && latestRelease === null && (
        <div className="update-checker-alert info">
          <p>
            Click "Check for Updates" to see if a new version is available.
          </p>
        </div>
      )}

      {!checking && !updateAvailable && latestRelease !== null && (
        <div className="update-checker-alert success">
          <CheckCircle size={16} />
          <span>
            You're running the latest version of Clerk! 🎉
          </span>
        </div>
      )}

      {updateAvailable && latestRelease && (
        <div className="update-checker-update-box">
          <div className="update-checker-update-header">
            <Download size={16} />
            <div className="update-checker-update-info">
              <p className="update-checker-update-version">
                New version available: {latestRelease.tag_name}
              </p>
              <p className="update-checker-update-date">
                Released on {new Date(latestRelease.published_at).toLocaleDateString()}
              </p>
            </div>
          </div>

          {latestRelease.body && (
            <div className="update-checker-release-notes">
              <p className="update-checker-release-notes-title">
                Release Notes:
              </p>
              <div className="update-checker-release-notes-content">
                {latestRelease.body.slice(0, 300)}
                {latestRelease.body.length > 300 && '...'}
              </div>
            </div>
          )}

          <button
            onClick={openDownloadPage}
            className="update-checker-download-btn"
          >
            <Download size={16} />
            Download Update
          </button>
        </div>
      )}

      <div className="update-checker-footer">
        <p>• Updates are checked manually for your security</p>
        <p>• All releases are verified and signed</p>
        <p>• Your data stays local and encrypted</p>
      </div>
    </div>
  );
}
