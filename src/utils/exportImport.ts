/**
 * Export/Import Utilities
 * Provides functions for exporting and importing data in various formats
 */

import type { Variable, Environment, Project } from '../types/database';

export type ExportFormat = 'env' | 'json' | 'csv';

export interface ExportOptions {
  format: ExportFormat;
  includeComments?: boolean;
  sortKeys?: boolean;
}

export interface ProjectExportData {
  project: {
    name: string;
    description?: string;
  };
  environments: Array<{
    name: string;
    variables: Array<{
      key: string;
      value: string;
    }>;
  }>;
  exportedAt: string;
  version: string;
}

/**
 * Export variables to .env format
 */
export function exportToEnv(
  variables: Variable[],
  options: { includeComments?: boolean; sortKeys?: boolean } = {}
): string {
  const { includeComments = true, sortKeys = true } = options;
  
  let vars = [...variables];
  if (sortKeys) {
    vars.sort((a, b) => a.key.localeCompare(b.key));
  }
  
  const lines: string[] = [];
  
  if (includeComments) {
    lines.push('# Environment Variables');
    lines.push(`# Exported at: ${new Date().toISOString()}`);
    lines.push('');
  }
  
  for (const variable of vars) {
    // Escape values with spaces or special characters
    const value = variable.value.includes(' ') || variable.value.includes('#')
      ? `"${variable.value.replace(/"/g, '\\"')}"`
      : variable.value;
    
    lines.push(`${variable.key}=${value}`);
  }
  
  return lines.join('\n');
}

/**
 * Export variables to CSV format
 */
export function exportToCSV(variables: Variable[]): string {
  const lines: string[] = [];
  
  // Header
  lines.push('Key,Value');
  
  // Data rows
  for (const variable of variables) {
    // Escape CSV values
    const key = escapeCsvValue(variable.key);
    const value = escapeCsvValue(variable.value);
    lines.push(`${key},${value}`);
  }
  
  return lines.join('\n');
}

/**
 * Escape CSV values
 */
function escapeCsvValue(value: string): string {
  // If value contains comma, quotes, or newlines, wrap in quotes and escape existing quotes
  if (value.includes(',') || value.includes('"') || value.includes('\n')) {
    return `"${value.replace(/"/g, '""')}"`;
  }
  return value;
}

/**
 * Export project data to JSON format (full backup)
 */
export function exportProjectToJSON(
  project: Project,
  environments: Environment[],
  variablesByEnv: Map<number, Variable[]>
): string {
  const data: ProjectExportData = {
    project: {
      name: project.name,
      ...(project.description && { description: project.description }),
    },
    environments: environments.map(env => ({
      name: env.name,
      variables: (variablesByEnv.get(env.id!) || []).map(v => ({
        key: v.key,
        value: v.value,
      })),
    })),
    exportedAt: new Date().toISOString(),
  version: '1.0.0',
  };
  
  return JSON.stringify(data, null, 2);
}

/**
 * Parse .env file content
 */
export function parseEnvFile(content: string): Array<{ key: string; value: string }> {
  const variables: Array<{ key: string; value: string }> = [];
  const lines = content.split('\n');
  
  for (let line of lines) {
    line = line.trim();
    
    // Skip empty lines and comments
    if (!line || line.startsWith('#')) {
      continue;
    }
    
    // Find the first = sign
    const equalIndex = line.indexOf('=');
    if (equalIndex === -1) {
      continue;
    }
    
    const key = line.substring(0, equalIndex).trim();
    let value = line.substring(equalIndex + 1).trim();
    
    // Remove quotes if present
    if ((value.startsWith('"') && value.endsWith('"')) || 
        (value.startsWith("'") && value.endsWith("'"))) {
      value = value.substring(1, value.length - 1);
      // Unescape quotes
      value = value.replace(/\\"/g, '"').replace(/\\'/g, "'");
    }
    
    if (key) {
      variables.push({ key, value });
    }
  }
  
  return variables;
}

/**
 * Parse CSV file content
 */
export function parseCSVFile(content: string): Array<{ key: string; value: string }> {
  const variables: Array<{ key: string; value: string }> = [];
  const lines = content.split('\n');
  
  // Skip header
  for (let i = 1; i < lines.length; i++) {
    const line = lines[i]?.trim();
    if (!line) continue;
    
    const parsed = parseCSVLine(line);
    if (parsed.length >= 2 && parsed[0] && parsed[1]) {
      variables.push({
        key: parsed[0],
        value: parsed[1],
      });
    }
  }
  
  return variables;
}

/**
 * Parse a single CSV line
 */
function parseCSVLine(line: string): string[] {
  const result: string[] = [];
  let current = '';
  let inQuotes = false;
  
  for (let i = 0; i < line.length; i++) {
    const char = line[i];
    const nextChar = line[i + 1];
    
    if (char === '"') {
      if (inQuotes && nextChar === '"') {
        // Escaped quote
        current += '"';
        i++; // Skip next quote
      } else {
        // Toggle quotes
        inQuotes = !inQuotes;
      }
    } else if (char === ',' && !inQuotes) {
      // Field separator
      result.push(current);
      current = '';
    } else {
      current += char;
    }
  }
  
  result.push(current);
  return result;
}

/**
 * Parse JSON backup file
 */
export function parseJSONBackup(content: string): ProjectExportData | null {
  try {
    const data = JSON.parse(content);
    
    // Validate structure
    if (!data.project || !data.environments || !Array.isArray(data.environments)) {
      return null;
    }
    
    return data as ProjectExportData;
  } catch {
    return null;
  }
}

/**
 * Validate import file format
 */
export function detectFileFormat(content: string): ExportFormat | null {
  const trimmed = content.trim();
  
  // Check JSON
  if (trimmed.startsWith('{')) {
    try {
      JSON.parse(content);
      return 'json';
    } catch {
      return null;
    }
  }
  
  // Check CSV (has header with "Key,Value")
  if (trimmed.startsWith('Key,Value') || trimmed.startsWith('key,value')) {
    return 'csv';
  }
  
  // Check .env format (KEY=VALUE pattern)
  const lines = trimmed.split('\n');
  let hasEnvPattern = false;
  
  for (const line of lines) {
    const trimmedLine = line.trim();
    if (!trimmedLine || trimmedLine.startsWith('#')) continue;
    
    if (trimmedLine.includes('=')) {
      hasEnvPattern = true;
      break;
    }
  }
  
  if (hasEnvPattern) {
    return 'env';
  }
  
  return null;
}

/**
 * Generate filename for export
 */
export function generateExportFilename(
  projectName: string,
  environmentName: string,
  format: ExportFormat
): string {
  const timestamp = new Date().toISOString().split('T')[0];
  const safeName = projectName.toLowerCase().replace(/[^a-z0-9]/g, '-');
  const safeEnv = environmentName.toLowerCase().replace(/[^a-z0-9]/g, '-');
  
  const extension = format === 'env' ? 'env' : format;
  return `${safeName}-${safeEnv}-${timestamp}.${extension}`;
}

/**
 * Generate filename for full project backup
 */
export function generateBackupFilename(projectName: string): string {
  const timestamp = new Date().toISOString().split('T')[0];
  const safeName = projectName.toLowerCase().replace(/[^a-z0-9]/g, '-');
  
  return `${safeName}-backup-${timestamp}.json`;
}
