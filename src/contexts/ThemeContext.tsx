import React, { createContext, useContext, useState, useEffect } from 'react';

export type ThemeColor = 'purple' | 'blue' | 'green' | 'orange' | 'pink';

interface ThemeContextType {
  isDarkMode: boolean;
  themeColor: ThemeColor;
  toggleDarkMode: () => void;
  setThemeColor: (color: ThemeColor) => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isDarkMode, setIsDarkMode] = useState(() => {
    // localStorage'dan dark mode tercihini yükle
    const saved = localStorage.getItem('clerk-dark-mode');
    return saved === 'true';
  });

  const [themeColor, setThemeColorState] = useState<ThemeColor>(() => {
    // localStorage'dan tema rengi tercihini yükle
    const saved = localStorage.getItem('clerk-theme-color') as ThemeColor;
    return saved || 'purple';
  });

  useEffect(() => {
    // Body'ye dark-mode class'ını ekle/çıkar
    if (isDarkMode) {
      document.body.classList.add('dark-mode');
    } else {
      document.body.classList.remove('dark-mode');
    }
    // localStorage'a kaydet
    localStorage.setItem('clerk-dark-mode', String(isDarkMode));
  }, [isDarkMode]);

  useEffect(() => {
    // Eski tema renklerini temizle
    document.body.classList.remove('theme-purple', 'theme-blue', 'theme-green', 'theme-orange', 'theme-pink');
    // Yeni tema rengini ekle
    document.body.classList.add(`theme-${themeColor}`);
    // localStorage'a kaydet
    localStorage.setItem('clerk-theme-color', themeColor);
  }, [themeColor]);

  const toggleDarkMode = () => {
    setIsDarkMode(prev => !prev);
  };

  const setThemeColor = (color: ThemeColor) => {
    setThemeColorState(color);
  };

  return (
    <ThemeContext.Provider value={{ isDarkMode, themeColor, toggleDarkMode, setThemeColor }}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
};
