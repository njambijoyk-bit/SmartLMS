import React, { useState, useEffect } from 'react';
import { developerAPI } from '../../lib/api';
import { VPATReport, VPATCriterion } from '../../types';
import Card from '../ui/Card';
import Button from '../ui/Button';
import Select from '../ui/Select';
import Badge from '../ui/Badge';

const VPATGenerator: React.FC = () => {
  const [reports, setReports] = useState<VPATReport[]>([]);
  const [loading, setLoading] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [selectedReport, setSelectedReport] = useState<VPATReport | null>(null);
  const [wcagLevel, setWcagLevel] = useState<'A' | 'AA' | 'AAA'>('AA');
  const [productName, setProductName] = useState('SmartLMS Platform');
  const [productVersion, setProductVersion] = useState('1.0.0');

  useEffect(() => {
    loadReports();
  }, []);

  const loadReports = async () => {
    try {
      setLoading(true);
      const data = await developerAPI.vpat.list();
      setReports(data);
    } catch (error) {
      console.error('Failed to load VPAT reports:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleGenerate = async () => {
    try {
      setGenerating(true);
      const report = await developerAPI.vpat.generate('smartlms-platform', wcagLevel);
      setReports([report, ...reports]);
      setSelectedReport(report);
    } catch (error) {
      console.error('Failed to generate VPAT report:', error);
    } finally {
      setGenerating(false);
    }
  };

  const handleExportPDF = async (reportId: string) => {
    try {
      const blob = await developerAPI.vpat.exportPDF(reportId);
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `vpat-report-${reportId}.pdf`;
      a.click();
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Failed to export PDF:', error);
    }
  };

  const getConformanceColor = (conformance: string) => {
    switch (conformance) {
      case 'supports': return 'success';
      case 'partially_supports': return 'warning';
      case 'does_not_support': return 'danger';
      default: return 'secondary';
    }
  };

  const getConformanceLabel = (conformance: string) => {
    switch (conformance) {
      case 'supports': return 'Supports';
      case 'partially_supports': return 'Partially Supports';
      case 'does_not_support': return 'Does Not Support';
      default: return 'N/A';
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">VPAT Accessibility Report</h1>
          <p className="text-gray-600 mt-1">Generate WCAG 2.1 compliance documentation</p>
        </div>
        <Button 
          onClick={handleGenerate} 
          loading={generating}
          className="bg-blue-600 hover:bg-blue-700"
        >
          Generate New Report
        </Button>
      </div>

      {/* Configuration */}
      <Card>
        <h3 className="text-lg font-semibold mb-4">Report Configuration</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Product Name
            </label>
            <input
              type="text"
              value={productName}
              onChange={(e) => setProductName(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Product Version
            </label>
            <input
              type="text"
              value={productVersion}
              onChange={(e) => setProductVersion(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              WCAG Level
            </label>
            <Select
              value={wcagLevel}
              onChange={(e) => setWcagLevel(e.target.value as 'A' | 'AA' | 'AAA')}
              options={[
                { value: 'A', label: 'Level A' },
                { value: 'AA', label: 'Level AA (Recommended)' },
                { value: 'AAA', label: 'Level AAA' },
              ]}
            />
          </div>
        </div>
      </Card>

      {/* Reports List */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-1">
          <Card>
            <h3 className="text-lg font-semibold mb-4">Generated Reports</h3>
            {loading ? (
              <div className="text-center py-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
                <p className="mt-2 text-gray-600">Loading reports...</p>
              </div>
            ) : reports.length === 0 ? (
              <p className="text-gray-500 text-center py-8">No reports generated yet</p>
            ) : (
              <div className="space-y-2">
                {reports.map((report) => (
                  <button
                    key={report.id}
                    onClick={() => setSelectedReport(report)}
                    className={`w-full text-left p-3 rounded-lg transition-colors ${
                      selectedReport?.id === report.id
                        ? 'bg-blue-50 border-blue-200 border'
                        : 'hover:bg-gray-50 border border-transparent'
                    }`}
                  >
                    <div className="flex justify-between items-start">
                      <div>
                        <p className="font-medium text-gray-900">{report.productName}</p>
                        <p className="text-sm text-gray-500">v{report.productVersion}</p>
                      </div>
                      <Badge variant={report.status === 'published' ? 'success' : 'secondary'}>
                        {report.status}
                      </Badge>
                    </div>
                    <div className="mt-2 flex items-center justify-between">
                      <span className="text-sm text-gray-600">
                        Score: {report.overallScore}%
                      </span>
                      <span className="text-xs text-gray-500">
                        {new Date(report.reportDate).toLocaleDateString()}
                      </span>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </Card>
        </div>

        {/* Report Details */}
        <div className="lg:col-span-2">
          {selectedReport ? (
            <Card>
              <div className="flex justify-between items-start mb-6">
                <div>
                  <h3 className="text-lg font-semibold">{selectedReport.productName}</h3>
                  <p className="text-gray-600">Version {selectedReport.productVersion}</p>
                  <p className="text-sm text-gray-500 mt-1">
                    Generated: {new Date(selectedReport.reportDate).toLocaleString()}
                  </p>
                </div>
                <div className="flex space-x-2">
                  <Button
                    onClick={() => handleExportPDF(selectedReport.id)}
                    variant="secondary"
                  >
                    Export PDF
                  </Button>
                  <Badge variant={selectedReport.wcagLevel === 'AAA' ? 'success' : 'primary'}>
                    WCAG {selectedReport.wcagLevel}
                  </Badge>
                </div>
              </div>

              {/* Overall Score */}
              <div className="mb-6">
                <div className="flex justify-between items-center mb-2">
                  <span className="text-sm font-medium text-gray-700">Overall Compliance Score</span>
                  <span className="text-2xl font-bold text-blue-600">{selectedReport.overallScore}%</span>
                </div>
                <div className="w-full bg-gray-200 rounded-full h-3">
                  <div
                    className={`h-3 rounded-full transition-all ${
                      selectedReport.overallScore >= 90 ? 'bg-green-500' :
                      selectedReport.overallScore >= 70 ? 'bg-yellow-500' : 'bg-red-500'
                    }`}
                    style={{ width: `${selectedReport.overallScore}%` }}
                  ></div>
                </div>
              </div>

              {/* Criteria */}
              <div className="space-y-4">
                <h4 className="font-semibold text-gray-900">Accessibility Criteria</h4>
                <div className="space-y-3 max-h-96 overflow-y-auto">
                  {selectedReport.criteria.map((criterion) => (
                    <div
                      key={criterion.id}
                      className="border border-gray-200 rounded-lg p-4"
                    >
                      <div className="flex justify-between items-start">
                        <div className="flex-1">
                          <div className="flex items-center space-x-2">
                            <span className="text-sm font-medium text-gray-900">
                              {criterion.criterion}
                            </span>
                            <Badge variant="secondary" size="sm">
                              {criterion.standard.replace(/_/g, ' ')}
                            </Badge>
                          </div>
                          <p className="text-sm text-gray-600 mt-1">{criterion.notes}</p>
                          {criterion.remediation && (
                            <p className="text-sm text-orange-600 mt-2">
                              <strong>Remediation:</strong> {criterion.remediation}
                            </p>
                          )}
                        </div>
                        <Badge variant={getConformanceColor(criterion.conformance)}>
                          {getConformanceLabel(criterion.conformance)}
                        </Badge>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </Card>
          ) : (
            <Card>
              <div className="text-center py-12">
                <div className="text-gray-400 mb-4">
                  <svg className="mx-auto h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                  </svg>
                </div>
                <h3 className="text-lg font-medium text-gray-900">No Report Selected</h3>
                <p className="text-gray-500 mt-1">Select a report from the list or generate a new one</p>
              </div>
            </Card>
          )}
        </div>
      </div>
    </div>
  );
};

export default VPATGenerator;
