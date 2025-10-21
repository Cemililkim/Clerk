/**
 * Backup Utilities
 * Handles vault backup creation and restoration
 */

export interface BackupMetadata {
  version: string;
  createdAt: string;
  vaultName: string;
  projectCount: number;
  environmentCount: number;
  variableCount: number;
}

export interface BackupFile {
  metadata: BackupMetadata;
  vaultData: string; // Base64 encoded encrypted vault file
  databaseData: string; // Base64 encoded encrypted database file
}

/**
 * Format backup filename with timestamp
 */
export function generateBackupFilename(vaultName: string = 'clerk'): string {
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-').split('T');
  const date = timestamp[0] || '';
  const time = timestamp[1]?.split('-')[0] || ''; // HH-MM-SS
  
  return `clerk-backup-${vaultName}-${date}-${time}.clerkbackup`;
}

/**
 * Parse backup filename to extract metadata
 */
export function parseBackupFilename(filename: string): { vaultName?: string; date?: string } | null {
  const match = filename.match(/clerk-backup-(.+)-(\d{4}-\d{2}-\d{2})-(\d{2}-\d{2}-\d{2})\.clerkbackup/);
  
  if (!match || !match[1] || !match[2] || !match[3]) {
    return null;
  }
  
  return {
    vaultName: match[1],
    date: `${match[2]} ${match[3].replace(/-/g, ':')}`,
  };
}

/**
 * Validate backup file structure
 */
export function validateBackupFile(data: unknown): data is BackupFile {
  if (typeof data !== 'object' || data === null) {
    return false;
  }
  
  const backup = data as Partial<BackupFile>;
  
  // Check metadata
  if (!backup.metadata || typeof backup.metadata !== 'object') {
    return false;
  }
  
  const metadata = backup.metadata;
  if (
    typeof metadata.version !== 'string' ||
    typeof metadata.createdAt !== 'string' ||
    typeof metadata.vaultName !== 'string' ||
    typeof metadata.projectCount !== 'number' ||
    typeof metadata.environmentCount !== 'number' ||
    typeof metadata.variableCount !== 'number'
  ) {
    return false;
  }
  
  // Check data
  if (
    typeof backup.vaultData !== 'string' ||
    typeof backup.databaseData !== 'string'
  ) {
    return false;
  }
  
  return true;
}

/**
 * Format file size for display
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';
  
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
}

/**
 * Format date for display
 */
export function formatBackupDate(dateString: string): string {
  const date = new Date(dateString);
  
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);
  const diffHours = Math.floor(diffMs / 3600000);
  const diffDays = Math.floor(diffMs / 86400000);
  
  if (diffMins < 1) {
    return 'Just now';
  } else if (diffMins < 60) {
    return `${diffMins} minute${diffMins !== 1 ? 's' : ''} ago`;
  } else if (diffHours < 24) {
    return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`;
  } else if (diffDays < 7) {
    return `${diffDays} day${diffDays !== 1 ? 's' : ''} ago`;
  } else {
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }
}

/**
 * Estimate backup size based on counts
 */
export function estimateBackupSize(
  projectCount: number,
  environmentCount: number,
  variableCount: number
): string {
  // Rough estimation:
  // Base overhead: ~10KB
  // Per project: ~500 bytes
  // Per environment: ~300 bytes
  // Per variable: ~200 bytes (average)
  
  const baseSize = 10240; // 10KB
  const estimatedBytes = 
    baseSize +
    (projectCount * 500) +
    (environmentCount * 300) +
    (variableCount * 200);
  
  return formatFileSize(estimatedBytes);
}
