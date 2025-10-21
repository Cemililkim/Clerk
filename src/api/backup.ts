import { invoke } from '@tauri-apps/api/core';

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
  vaultData: string;     // Base64 encoded
  databaseData: string;  // Base64 encoded
}

export interface BackupInfo {
  filename: string;
  metadata: BackupMetadata;
  fileSize: number;
}

/**
 * Creates a backup of the current vault and database
 */
export async function createBackup(): Promise<BackupFile> {
  return await invoke<BackupFile>('create_backup');
}

/**
 * Restores a backup to the specified vault and database paths
 */
export async function restoreBackup(
  backupJson: string
): Promise<string> {
  return await invoke<string>('restore_backup', {
    backupJson,
  });
}

/**
 * Extracts metadata from a backup file without fully restoring it
 */
export async function getBackupInfo(backupJson: string): Promise<BackupMetadata> {
  return await invoke<BackupMetadata>('get_backup_info', {
    backupJson,
  });
}

/**
 * Validates a backup file structure
 */
export async function validateBackupFile(backupJson: string): Promise<boolean> {
  return await invoke<boolean>('validate_backup_file', {
    backupJson,
  });
}

/**
 * Saves a backup to a file (using the frontend utility)
 */
export async function saveBackupToFile(
  backup: BackupFile,
  filePath: string
): Promise<void> {
  const backupJson = JSON.stringify(backup, null, 2);
  await invoke('write_file_content', {
    filePath,
    content: backupJson,
  });
}

/**
 * Reads a backup from a file (using the frontend utility)
 */
export async function loadBackupFromFile(filePath: string): Promise<BackupFile> {
  const content = await invoke<string>('read_file_content', {
    filePath,
  });
  return JSON.parse(content);
}
