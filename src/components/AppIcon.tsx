import React from 'react';
import { useTheme } from '../contexts/ThemeContext';
import './AppIcon.css';

interface AppIconProps {
  size?: number;
}

export const AppIcon: React.FC<AppIconProps> = ({ size = 40 }) => {
  useTheme(); // ensure theme provider is active so body class is set

  const style: React.CSSProperties = {
    width: size,
    height: size,
    display: 'inline-flex',
    alignItems: 'center',
    justifyContent: 'center',
    borderRadius: 8,
    overflow: 'hidden',
  };

  return (
    <div className="app-icon theme-bg" style={style}>
      {/* Inline SVG so we can control fills reliably */}
      <svg viewBox="0 0 512 512" width="70%" height="70%" aria-hidden focusable="false">
        <g transform="translate(0,10.000002)" fill="#ffffff">
          <path d="m 256,117.4 c 0,0 -21.6,25.55977 -37.8,32.03965 -16.2,6.47988 -59.4,-2.16035 -59.4,-2.16035 0,0 -8.93701,52.00376 0,81.5207 16.92627,55.90368 97.2,145.8 97.2,145.8 0,0 80.27373,-89.89632 97.2,-145.8 8.93701,-29.51694 0,-81.5207 0,-81.5207 0,0 -42.3135,9.13867 -59.4,2.16035 C 278.13805,143.04313 256,117.4 256,117.4 Z" />
          <path d="m 256,69.660156 c 0,0 -29.04031,34.364324 -50.82031,43.076174 -21.78,8.71184 -79.85938,-2.90625 -79.85938,-2.90625 0,0 -12.01531,76.80634 0,116.49023 C 148.07675,301.4797 256,422.33984 256,422.33984 c 0,0 107.92325,-120.86014 130.67969,-196.01953 12.01531,-39.68389 0,-116.49023 0,-116.49023 0,0 -56.88753,12.28821 -79.85938,2.90625 C 285.76368,104.13657 256,69.660156 256,69.660156 Z M 256,102 c 0,0 24.59783,28.49237 42,35.59961 18.985,7.75369 66,-2.40039 66,-2.40039 0,0 9.93001,60.00418 0,92.80078 -18.80697,62.1152 -108,162 -108,162 0,0 -89.19303,-99.8848 -108,-162 -9.93001,-32.7966 0,-92.80078 0,-92.80078 0,0 48,9.60026 66,2.40039 C 232,130.39974 256,102 256,102 Z" />
        </g>
      </svg>
    </div>
  );
};

export default AppIcon;
