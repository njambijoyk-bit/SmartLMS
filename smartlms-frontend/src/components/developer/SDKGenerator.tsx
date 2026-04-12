import React, { useState } from 'react';
import { developerAPI } from '../../lib/api';
import Card from '../ui/Card';
import Button from '../ui/Button';
import Select from '../ui/Select';

const SDKGenerator: React.FC = () => {
  const [generating, setGenerating] = useState(false);
  const [generatedPackage, setGeneratedPackage] = useState<any>(null);
  const [config, setConfig] = useState({
    language: 'typescript',
    packageName: 'smartlms-sdk',
    version: '1.0.0',
    includeExamples: true,
    includeTests: true,
    apiVersion: 'v1',
  });

  const languages = [
    { value: 'typescript', label: 'TypeScript / JavaScript' },
    { value: 'python', label: 'Python' },
    { value: 'java', label: 'Java' },
    { value: 'csharp', label: 'C# (.NET)' },
    { value: 'go', label: 'Go' },
    { value: 'php', label: 'PHP' },
  ];

  const handleGenerate = async () => {
    try {
      setGenerating(true);
      const pkg = await developerAPI.sdk.generate(config);
      setGeneratedPackage(pkg);
    } catch (error) {
      console.error('Failed to generate SDK:', error);
    } finally {
      setGenerating(false);
    }
  };

  const handleDownload = async () => {
    if (!generatedPackage) return;
    try {
      const blob = await developerAPI.sdk.downloadPackage(generatedPackage.id);
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `${generatedPackage.packageName}-${generatedPackage.version}.zip`;
      a.click();
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Failed to download package:', error);
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">SDK Generator</h1>
        <p className="text-gray-600 mt-1">Generate client libraries for your preferred programming language</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Configuration */}
        <Card>
          <h3 className="text-lg font-semibold mb-4">SDK Configuration</h3>
          <form className="space-y-4" onSubmit={(e) => { e.preventDefault(); handleGenerate(); }}>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Programming Language
              </label>
              <Select
                value={config.language}
                onChange={(e) => setConfig({ ...config, language: e.target.value })}
                options={languages}
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Package Name
              </label>
              <input
                type="text"
                value={config.packageName}
                onChange={(e) => setConfig({ ...config, packageName: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Version
              </label>
              <input
                type="text"
                value={config.version}
                onChange={(e) => setConfig({ ...config, version: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                API Version
              </label>
              <input
                type="text"
                value={config.apiVersion}
                onChange={(e) => setConfig({ ...config, apiVersion: e.target.value })}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div className="flex items-center space-x-4">
              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={config.includeExamples}
                  onChange={(e) => setConfig({ ...config, includeExamples: e.target.checked })}
                  className="h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="ml-2 text-sm text-gray-700">Include Examples</span>
              </label>

              <label className="flex items-center">
                <input
                  type="checkbox"
                  checked={config.includeTests}
                  onChange={(e) => setConfig({ ...config, includeTests: e.target.checked })}
                  className="h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="ml-2 text-sm text-gray-700">Include Tests</span>
              </label>
            </div>

            <Button
              type="submit"
              loading={generating}
              className="w-full bg-blue-600 hover:bg-blue-700"
            >
              Generate SDK
            </Button>
          </form>
        </Card>

        {/* Output */}
        <Card>
          <h3 className="text-lg font-semibold mb-4">Generated Package</h3>
          {generatedPackage ? (
            <div className="space-y-4">
              <div className="bg-green-50 border border-green-200 rounded-lg p-4">
                <div className="flex items-center">
                  <svg className="h-6 w-6 text-green-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  <span className="font-medium text-green-800">SDK Generated Successfully!</span>
                </div>
              </div>

              <div className="space-y-2">
                <div className="flex justify-between py-2 border-b border-gray-200">
                  <span className="text-sm text-gray-600">Package:</span>
                  <code className="text-sm font-mono">{generatedPackage.packageName}</code>
                </div>
                <div className="flex justify-between py-2 border-b border-gray-200">
                  <span className="text-sm text-gray-600">Language:</span>
                  <span className="text-sm font-medium">{generatedPackage.language}</span>
                </div>
                <div className="flex justify-between py-2 border-b border-gray-200">
                  <span className="text-sm text-gray-600">Version:</span>
                  <span className="text-sm font-medium">{generatedPackage.version}</span>
                </div>
                <div className="flex justify-between py-2 border-b border-gray-200">
                  <span className="text-sm text-gray-600">Size:</span>
                  <span className="text-sm font-medium">
                    {(generatedPackage.size / 1024).toFixed(2)} KB
                  </span>
                </div>
                <div className="flex justify-between py-2 border-b border-gray-200">
                  <span className="text-sm text-gray-600">Generated:</span>
                  <span className="text-sm font-medium">
                    {new Date(generatedPackage.createdAt).toLocaleString()}
                  </span>
                </div>
              </div>

              <div className="pt-4">
                <h4 className="text-sm font-medium text-gray-700 mb-2">Installation</h4>
                <div className="bg-gray-900 text-gray-100 rounded-lg p-4 overflow-x-auto">
                  <code className="text-sm font-mono">
                    {generatedPackage.language === 'typescript' && `npm install ${generatedPackage.packageName}`}
                    {generatedPackage.language === 'python' && `pip install ${generatedPackage.packageName}`}
                    {generatedPackage.language === 'java' && `<dependency>\n  <groupId>com.smartlms</groupId>\n  <artifactId>${generatedPackage.packageName}</artifactId>\n  <version>${generatedPackage.version}</version>\n</dependency>`}
                    {generatedPackage.language === 'csharp' && `dotnet add package ${generatedPackage.packageName}`}
                    {generatedPackage.language === 'go' && `go get github.com/smartlms/${generatedPackage.packageName}`}
                    {generatedPackage.language === 'php' && `composer require smartlms/${generatedPackage.packageName}`}
                  </code>
                </div>
              </div>

              <Button onClick={handleDownload} className="w-full bg-blue-600 hover:bg-blue-700">
                Download Package
              </Button>
            </div>
          ) : (
            <div className="text-center py-12">
              <div className="text-gray-400 mb-4">
                <svg className="mx-auto h-12 w-12" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                </svg>
              </div>
              <h3 className="text-lg font-medium text-gray-900">No SDK Generated</h3>
              <p className="text-gray-500 mt-1">Configure your SDK and click generate to create a package</p>
            </div>
          )}
        </Card>
      </div>

      {/* Features */}
      <Card>
        <h3 className="text-lg font-semibold mb-4">What's Included</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center mb-2">
              <svg className="h-5 w-5 text-blue-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
              <span className="font-medium text-gray-900">Type-Safe Client</span>
            </div>
            <p className="text-sm text-gray-600">
              Fully typed interfaces with autocomplete support in your IDE
            </p>
          </div>

          <div className="p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center mb-2">
              <svg className="h-5 w-5 text-blue-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
              <span className="font-medium text-gray-900">API Documentation</span>
            </div>
            <p className="text-sm text-gray-600">
              Complete JSDoc/docstring documentation for all methods
            </p>
          </div>

          <div className="p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center mb-2">
              <svg className="h-5 w-5 text-blue-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
              <span className="font-medium text-gray-900">Error Handling</span>
            </div>
            <p className="text-sm text-gray-600">
              Built-in error handling with descriptive error messages
            </p>
          </div>

          {config.includeExamples && (
            <div className="p-4 bg-gray-50 rounded-lg">
              <div className="flex items-center mb-2">
                <svg className="h-5 w-5 text-green-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="font-medium text-gray-900">Code Examples</span>
              </div>
              <p className="text-sm text-gray-600">
                Ready-to-use examples for common use cases
              </p>
            </div>
          )}

          {config.includeTests && (
            <div className="p-4 bg-gray-50 rounded-lg">
              <div className="flex items-center mb-2">
                <svg className="h-5 w-5 text-green-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="font-medium text-gray-900">Test Suite</span>
              </div>
              <p className="text-sm text-gray-600">
                Comprehensive test suite with example tests
              </p>
            </div>
          )}

          <div className="p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center mb-2">
              <svg className="h-5 w-5 text-blue-600 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
              <span className="font-medium text-gray-900">Authentication</span>
            </div>
            <p className="text-sm text-gray-600">
              OAuth 2.0 and API key authentication built-in
            </p>
          </div>
        </div>
      </Card>
    </div>
  );
};

export default SDKGenerator;
