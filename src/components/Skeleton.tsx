import React from 'react';
import './Skeleton.css';

interface SkeletonProps {
  variant?: 'text' | 'circular' | 'rectangular';
  width?: string | number;
  height?: string | number;
  className?: string;
}

export const Skeleton: React.FC<SkeletonProps> = ({ 
  variant = 'text', 
  width, 
  height,
  className = ''
}) => {
  const style: React.CSSProperties = {
    width: width,
    height: height || (variant === 'text' ? '1em' : undefined),
  };

  return (
    <div 
      className={`skeleton skeleton-${variant} ${className}`}
      style={style}
    />
  );
};

// Project List Skeleton
export const ProjectListSkeleton: React.FC = () => {
  return (
    <div className="project-list-skeleton">
      {[1, 2, 3].map((i) => (
        <div key={i} className="project-skeleton-item">
          <Skeleton variant="circular" width={40} height={40} />
          <div className="project-skeleton-content">
            <Skeleton variant="text" width="60%" height={16} />
            <Skeleton variant="text" width="80%" height={14} />
          </div>
        </div>
      ))}
    </div>
  );
};

// Variable Table Skeleton
export const VariableTableSkeleton: React.FC = () => {
  return (
    <div className="variable-table-skeleton">
      <div className="variable-table-skeleton-header">
        <Skeleton variant="text" width={100} height={14} />
        <Skeleton variant="text" width={150} height={14} />
        <Skeleton variant="text" width={80} height={14} />
      </div>
      {[1, 2, 3, 4, 5].map((i) => (
        <div key={i} className="variable-table-skeleton-row">
          <Skeleton variant="rectangular" width={120} height={28} />
          <Skeleton variant="text" width="70%" height={16} />
          <div className="variable-table-skeleton-actions">
            <Skeleton variant="circular" width={32} height={32} />
            <Skeleton variant="circular" width={32} height={32} />
            <Skeleton variant="circular" width={32} height={32} />
          </div>
        </div>
      ))}
    </div>
  );
};

// Environment Tabs Skeleton
export const EnvironmentTabsSkeleton: React.FC = () => {
  return (
    <div className="environment-tabs-skeleton">
      {[1, 2, 3].map((i) => (
        <Skeleton key={i} variant="rectangular" width={120} height={40} />
      ))}
    </div>
  );
};
