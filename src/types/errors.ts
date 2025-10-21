/**
 * Custom error classes for Clerk application
 */

export class ClerkError extends Error {
  constructor(
    message: string,
    public readonly code: string
  ) {
    super(message);
    this.name = 'ClerkError';
  }
}

export class VaultError extends ClerkError {
  constructor(message: string, code = 'VAULT_ERROR') {
    super(message, code);
    this.name = 'VaultError';
  }
}

export class EncryptionError extends ClerkError {
  constructor(message: string, code = 'ENCRYPTION_ERROR') {
    super(message, code);
    this.name = 'EncryptionError';
  }
}

export class DecryptionError extends ClerkError {
  constructor(message: string, code = 'DECRYPTION_ERROR') {
    super(message, code);
    this.name = 'DecryptionError';
  }
}

export class InvalidPasswordError extends ClerkError {
  constructor(message = 'Invalid password', code = 'INVALID_PASSWORD') {
    super(message, code);
    this.name = 'InvalidPasswordError';
  }
}

export class WeakPasswordError extends ClerkError {
  constructor(message = 'Password is too weak', code = 'WEAK_PASSWORD') {
    super(message, code);
    this.name = 'WeakPasswordError';
  }
}

export class StorageError extends ClerkError {
  constructor(message: string, code = 'STORAGE_ERROR') {
    super(message, code);
    this.name = 'StorageError';
  }
}

export class KeychainError extends ClerkError {
  constructor(message: string, code = 'KEYCHAIN_ERROR') {
    super(message, code);
    this.name = 'KeychainError';
  }
}
