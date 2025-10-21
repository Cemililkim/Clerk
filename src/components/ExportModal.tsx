import React, { useState } from 'react';
import { Download, FileJson, FileText, Table, X } from 'lucide-react';
import type { ExportFormat } from '../utils/exportImport';
import './ExportModal.css';

interface ExportModalProps {
  isOpen: boolean;
  onClose: () => void;
  onExport: (format: ExportFormat, options: { includeComments?: boolean; sortKeys?: boolean }) => void;
  entityName: string; // e.g., "Production Environment"
}

export const ExportModal: React.FC<ExportModalProps> = ({ 
  isOpen, 
  onClose, 
  onExport,
  entityName 
}) => {
  const [selectedFormat, setSelectedFormat] = useState<ExportFormat>('env');
  const [includeComments, setIncludeComments] = useState(true);
  const [sortKeys, setSortKeys] = useState(true);

  if (!isOpen) return null;

  const handleExport = () => {
    onExport(selectedFormat, { includeComments, sortKeys });
    onClose();
  };

  const handleOverlayClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  const formats = [
    {
      value: 'env' as ExportFormat,
      icon: FileText,
      label: '.env File',
      description: 'Standard environment variables format',
      recommended: true,
    },
    {
      value: 'json' as ExportFormat,
      icon: FileJson,
      label: 'JSON',
      description: 'Structured data format with metadata',
      recommended: false,
    },
    {
      value: 'csv' as ExportFormat,
      icon: Table,
      label: 'CSV',
      description: 'Spreadsheet-compatible format',
      recommended: false,
    },
  ];

  return (
    <div className="export-modal-overlay" onClick={handleOverlayClick}>
      <div className="export-modal">
        <div className="export-modal-header">
          <h2>
            <Download size={20} />
            Export Variables
          </h2>
          <button 
            className="export-modal-close"
            onClick={onClose}
            aria-label="Close modal"
          >
            <X size={20} />
          </button>
        </div>

        <div className="export-modal-body">
          <p className="export-modal-subtitle">
            Export variables from <strong>{entityName}</strong>
          </p>

          <div className="export-modal-section">
            <label className="export-modal-section-label">Select Format</label>
            <div className="export-modal-format-grid">
              {formats.map((format) => {
                const Icon = format.icon;
                return (
                  <button
                    key={format.value}
                    className={`export-modal-format-option ${selectedFormat === format.value ? 'selected' : ''}`}
                    onClick={() => setSelectedFormat(format.value)}
                  >
                    <div className="export-modal-format-icon">
                      <Icon size={24} />
                    </div>
                    <div className="export-modal-format-info">
                      <div className="export-modal-format-label">
                        {format.label}
                        {format.recommended && (
                          <span className="export-modal-recommended-badge">Recommended</span>
                        )}
                      </div>
                      <div className="export-modal-format-description">
                        {format.description}
                      </div>
                    </div>
                  </button>
                );
              })}
            </div>
          </div>

          {selectedFormat === 'env' && (
            <div className="export-modal-section">
              <label className="export-modal-section-label">Options</label>
              <div className="export-modal-options">
                <label className="export-modal-checkbox">
                  <input
                    type="checkbox"
                    checked={includeComments}
                    onChange={(e) => setIncludeComments(e.target.checked)}
                  />
                  <span>Include comments (timestamps, metadata)</span>
                </label>
                <label className="export-modal-checkbox">
                  <input
                    type="checkbox"
                    checked={sortKeys}
                    onChange={(e) => setSortKeys(e.target.checked)}
                  />
                  <span>Sort keys alphabetically</span>
                </label>
              </div>
            </div>
          )}
        </div>

        <div className="export-modal-footer">
          <button
            type="button"
            className="export-modal-btn-cancel"
            onClick={onClose}
          >
            Cancel
          </button>
          <button
            type="button"
            className="export-modal-btn-export"
            onClick={handleExport}
          >
            <Download size={16} />
            Export
          </button>
        </div>
      </div>
    </div>
  );
};
