/**
 * Core domain types for Clerk application
 */

/**
 * Represents an environment variable key-value pair
 */
export interface EnvironmentVariable {
  key: string;
  value: string;
  description?: string;
  createdAt: number;
  updatedAt: number;
}

/**
 * Represents an environment (e.g., development, staging, production)
 */
export interface Environment {
  id: string;
  name: string;
  variables: Record<string, EnvironmentVariable>;
  createdAt: number;
  updatedAt: number;
}

/**
 * Represents a project with multiple environments
 */
export interface Project {
  id: string;
  name: string;
  description?: string;
  environments: Record<string, Environment>;
  createdAt: number;
  updatedAt: number;
}

/**
 * The main vault data structure
 */
export interface VaultData {
  version: number;
  projects: Record<string, Project>;
  createdAt: number;
  updatedAt: number;
}

/**
 * Encrypted data structure
 */
export interface EncryptedData {
  ciphertext: Uint8Array;
  iv: Uint8Array;
  algorithm: string;
  salt?: Uint8Array;
}

/**
 * Vault metadata (stored unencrypted)
 */
export interface VaultMetadata {
  version: number;
  createdAt: number;
  salt: Uint8Array;
}

/**
 * Stored vault file structure
 */
export interface StoredVault {
  metadata: VaultMetadata;
  encryptedData: EncryptedData;
}
