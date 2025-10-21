import { useEffect, useRef, useCallback } from 'react';

export interface UseInactivityTimerOptions {
  /** Timeout duration in milliseconds */
  timeout: number;
  /** Callback fired when timeout expires */
  onTimeout: () => void;
  /** Whether the timer is enabled */
  enabled?: boolean;
  /** Events that reset the timer (default: mousemove, keydown, click, scroll) */
  events?: string[];
}

/**
 * Hook to track user inactivity and fire a callback after a timeout period.
 * Resets the timer on any user interaction (mouse, keyboard, scroll).
 */
export function useInactivityTimer({
  timeout,
  onTimeout,
  enabled = true,
  events = ['mousemove', 'keydown', 'click', 'scroll', 'touchstart'],
}: UseInactivityTimerOptions) {
  const timeoutIdRef = useRef<number | null>(null);
  const lastActivityRef = useRef<number>(Date.now());

  // Get remaining time in milliseconds
  const getRemainingTime = useCallback(() => {
    const elapsed = Date.now() - lastActivityRef.current;
    return Math.max(0, timeout - elapsed);
  }, [timeout]);

  // Reset the inactivity timer
  const resetTimer = useCallback(() => {
    lastActivityRef.current = Date.now();

    // Clear existing timeout
    if (timeoutIdRef.current) {
      clearTimeout(timeoutIdRef.current);
    }

    // Set new timeout if enabled
    if (enabled && timeout > 0) {
      timeoutIdRef.current = setTimeout(() => {
        onTimeout();
      }, timeout);
    }
  }, [timeout, onTimeout, enabled]);

  // Activity handler - debounced
  const handleActivity = useCallback(() => {
    resetTimer();
  }, [resetTimer]);

  useEffect(() => {
    if (!enabled || timeout <= 0) {
      // Clear timer if disabled
      if (timeoutIdRef.current) {
        clearTimeout(timeoutIdRef.current);
        timeoutIdRef.current = null;
      }
      return;
    }

    // Start initial timer
    resetTimer();

    // Add event listeners for all specified events
    events.forEach((event) => {
      window.addEventListener(event, handleActivity);
    });

    // Cleanup
    return () => {
      if (timeoutIdRef.current) {
        clearTimeout(timeoutIdRef.current);
      }
      events.forEach((event) => {
        window.removeEventListener(event, handleActivity);
      });
    };
  }, [enabled, timeout, events, handleActivity, resetTimer]);

  return {
    /** Manually reset the timer */
    reset: resetTimer,
    /** Get remaining time in milliseconds */
    getRemainingTime,
  };
}
