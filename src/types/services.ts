/**
 * Service interface types
 */

import type { EncryptedData } from './vault';

/**
 * Encryption service interface
 */
export interface EncryptionService {
  encrypt(plaintext: Uint8Array, key: CryptoKey): Promise<EncryptedData>;
  decrypt(encrypted: EncryptedData, key: CryptoKey): Promise<Uint8Array>;
}

/**
 * Key derivation service interface
 */
export interface KeyDerivationService {
  deriveKey(password: string, salt: Uint8Array, iterations?: number): Promise<CryptoKey>;
  generateSalt(): Uint8Array;
}

/**
 * Storage service interface
 */
export interface StorageService {
  save(data: Uint8Array, path: string): Promise<void>;
  load(path: string): Promise<Uint8Array>;
  exists(path: string): Promise<boolean>;
  delete(path: string): Promise<void>;
}

/**
 * Keychain service interface (OS integration)
 */
export interface KeychainService {
  isAvailable(): Promise<boolean>;
  save(key: string, value: string): Promise<void>;
  load(key: string): Promise<string | null>;
  delete(key: string): Promise<void>;
}
