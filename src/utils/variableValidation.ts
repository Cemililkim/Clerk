/**
 * Variable Validation Utilities
 * Provides validation functions for different types of environment variables
 */

export type ValidationType = 'url' | 'email' | 'port' | 'json' | 'none';

export interface ValidationResult {
  isValid: boolean;
  message?: string;
  type: ValidationType;
}

/**
 * Auto-detect validation type based on variable key
 */
export function detectValidationType(key: string): ValidationType {
  const lowerKey = key.toLowerCase();
  
  // URL detection
  if (lowerKey.includes('url') || lowerKey.includes('endpoint') || lowerKey.includes('api')) {
    return 'url';
  }
  
  // Email detection
  if (lowerKey.includes('email') || lowerKey.includes('mail')) {
    return 'email';
  }
  
  // Port detection
  if (lowerKey.includes('port')) {
    return 'port';
  }
  
  // JSON detection
  if (lowerKey.includes('config') || lowerKey.includes('json') || lowerKey.includes('settings')) {
    return 'json';
  }
  
  return 'none';
}

/**
 * Validate URL format
 */
export function validateURL(value: string): ValidationResult {
  if (!value || value.trim() === '') {
    return { isValid: true, type: 'url' }; // Empty is valid (optional)
  }
  
  try {
    const url = new URL(value);
    const isValid = url.protocol === 'http:' || url.protocol === 'https:';
    
    if (!isValid) {
      return {
        isValid: false,
        message: 'URL must start with http:// or https://',
        type: 'url'
      };
    }
    
    return { isValid: true, type: 'url' };
  } catch {
    return {
      isValid: false,
      message: 'Invalid URL format',
      type: 'url'
    };
  }
}

/**
 * Validate email format
 */
export function validateEmail(value: string): ValidationResult {
  if (!value || value.trim() === '') {
    return { isValid: true, type: 'email' }; // Empty is valid (optional)
  }
  
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  const isValid = emailRegex.test(value);
  
  if (!isValid) {
    return {
      isValid: false,
      message: 'Invalid email format',
      type: 'email'
    };
  }
  
  return { isValid: true, type: 'email' };
}

/**
 * Validate port number
 */
export function validatePort(value: string): ValidationResult {
  if (!value || value.trim() === '') {
    return { isValid: true, type: 'port' }; // Empty is valid (optional)
  }
  
  const port = parseInt(value, 10);
  const isValid = !isNaN(port) && port >= 0 && port <= 65535;
  
  if (!isValid) {
    return {
      isValid: false,
      message: 'Port must be between 0 and 65535',
      type: 'port'
    };
  }
  
  return { isValid: true, type: 'port' };
}

/**
 * Validate JSON format
 */
export function validateJSON(value: string): ValidationResult {
  if (!value || value.trim() === '') {
    return { isValid: true, type: 'json' }; // Empty is valid (optional)
  }
  
  try {
    JSON.parse(value);
    return { isValid: true, type: 'json' };
  } catch (err) {
    return {
      isValid: false,
      message: err instanceof Error ? err.message : 'Invalid JSON format',
      type: 'json'
    };
  }
}

/**
 * Main validation function that auto-detects type and validates
 */
export function validateVariable(key: string, value: string, explicitType?: ValidationType): ValidationResult {
  const type = explicitType || detectValidationType(key);
  
  switch (type) {
    case 'url':
      return validateURL(value);
    case 'email':
      return validateEmail(value);
    case 'port':
      return validatePort(value);
    case 'json':
      return validateJSON(value);
    default:
      return { isValid: true, type: 'none' };
  }
}
