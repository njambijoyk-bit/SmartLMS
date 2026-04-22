import React, { useState, useEffect } from 'react';
import { Button } from '../ui/Button';
import { Card, CardHeader, CardTitle, CardContent } from '../ui/Card';
import { Alert, AlertDescription } from '../ui/Alert';

interface ExecutionResult {
  id: string;
  status: 'success' | 'runtime_error' | 'timeout' | 'memory_limit_exceeded' | 'compilation_error' | 'internal_error';
  stdout: string;
  stderr: string;
  exit_code: number | null;
  execution_time_ms: number;
  memory_used_kb: number;
  error_message: string | null;
}

interface LanguageInfo {
  name: string;
  version: string;
  file_extension: string;
}

interface CodeSandboxProps {
  defaultLanguage?: string;
  onExecute?: (result: ExecutionResult) => void;
}

const LANGUAGE_TEMPLATES: Record<string, string> = {
  python: `# Write your Python code here\ndef hello_world():\n    print("Hello, World!")\n\nhello_world()`,
  java: `// Write your Java code here\npublic class Main {\n    public static void main(String[] args) {\n        System.out.println("Hello, World!");\n    }\n}`,
  cpp: `// Write your C++ code here\n#include <iostream>\nusing namespace std;\n\nint main() {\n    cout << "Hello, World!" << endl;\n    return 0;\n}`,
  c: `// Write your C code here\n#include <stdio.h>\n\nint main() {\n    printf("Hello, World!\\n");\n    return 0;\n}`,
  javascript: `// Write your JavaScript code here\nconsole.log("Hello, World!");`,
  rust: `// Write your Rust code here\nfn main() {\n    println!("Hello, World!");\n}`,
  go: `// Write your Go code here\npackage main\n\nimport "fmt"\n\nfunc main() {\n    fmt.Println("Hello, World!")\n}`,
};

export const CodeSandbox: React.FC<CodeSandboxProps> = ({ 
  defaultLanguage = 'python',
  onExecute 
}) => {
  const [language, setLanguage] = useState<string>(defaultLanguage);
  const [code, setCode] = useState<string>(LANGUAGE_TEMPLATES[defaultLanguage]);
  const [input, setInput] = useState<string>('');
  const [isExecuting, setIsExecuting] = useState(false);
  const [result, setResult] = useState<ExecutionResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [languages, setLanguages] = useState<LanguageInfo[]>([]);
  const [showInput, setShowInput] = useState(false);

  useEffect(() => {
    fetchLanguages();
  }, []);

  useEffect(() => {
    if (LANGUAGE_TEMPLATES[language]) {
      setCode(LANGUAGE_TEMPLATES[language]);
    }
  }, [language]);

  const fetchLanguages = async () => {
    try {
      const response = await fetch('/api/code-sandbox/languages');
      if (response.ok) {
        const data = await response.json();
        setLanguages(data);
      }
    } catch (err) {
      console.error('Failed to fetch languages:', err);
    }
  };

  const handleExecute = async () => {
    setIsExecuting(true);
    setError(null);
    setResult(null);

    try {
      const response = await fetch('/api/code-sandbox/execute', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          language,
          code,
          input: showInput ? input : undefined,
          timeout_ms: 5000,
          memory_limit_mb: 128,
          cpu_limit: 0.5,
        }),
      });

      if (!response.ok) {
        throw new Error(await response.text());
      }

      const data: ExecutionResult = await response.json();
      setResult(data);
      
      if (onExecute) {
        onExecute(data);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Execution failed');
    } finally {
      setIsExecuting(false);
    }
  };

  const handleStop = async () => {
    if (result?.id) {
      try {
        await fetch(`/api/code-sandbox/stop/${result.id}`, { method: 'POST' });
      } catch (err) {
        console.error('Failed to stop execution:', err);
      }
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'success': return 'text-green-600 bg-green-50 border-green-200';
      case 'timeout': return 'text-yellow-600 bg-yellow-50 border-yellow-200';
      case 'compilation_error': return 'text-red-600 bg-red-50 border-red-200';
      case 'runtime_error': return 'text-orange-600 bg-orange-50 border-orange-200';
      default: return 'text-gray-600 bg-gray-50 border-gray-200';
    }
  };

  const getStatusLabel = (status: string) => {
    switch (status) {
      case 'success': return '✓ Success';
      case 'timeout': return '⏱ Timeout';
      case 'compilation_error': return '✗ Compilation Error';
      case 'runtime_error': return '✗ Runtime Error';
      case 'memory_limit_exceeded': return '✗ Memory Limit';
      case 'internal_error': return '✗ Internal Error';
      default: return status;
    }
  };

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <span>Code Playground</span>
          <select
            value={language}
            onChange={(e) => setLanguage(e.target.value)}
            className="px-3 py-1 border rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-primary"
          >
            {languages.length > 0 ? (
              languages.map((lang) => (
                <option key={lang.name} value={lang.name}>
                  {lang.name} {lang.version}
                </option>
              ))
            ) : (
              <>
                <option value="python">Python 3.11</option>
                <option value="java">Java 17</option>
                <option value="cpp">C++ 17</option>
                <option value="c">C 11</option>
                <option value="javascript">JavaScript (Node 18)</option>
                <option value="rust">Rust 1.70</option>
                <option value="go">Go 1.21</option>
              </>
            )}
          </select>
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {/* Toolbar */}
          <div className="flex items-center gap-2">
            <Button 
              onClick={handleExecute} 
              disabled={isExecuting}
              className="px-6"
            >
              {isExecuting ? 'Running...' : '▶ Run Code'}
            </Button>
            
            {isExecuting && (
              <Button 
                variant="destructive" 
                onClick={handleStop}
                size="sm"
              >
                ⏹ Stop
              </Button>
            )}
            
            <Button
              variant="outline"
              onClick={() => setShowInput(!showInput)}
              size="sm"
            >
              {showInput ? 'Hide Input' : '📥 Custom Input'}
            </Button>
            
            <div className="ml-auto text-sm text-muted-foreground">
              Press Ctrl+Enter to run
            </div>
          </div>

          {/* Custom Input */}
          {showInput && (
            <div>
              <label className="block text-sm font-medium mb-1">Standard Input</label>
              <textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                placeholder="Enter input for your program..."
                className="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary font-mono text-sm"
                rows={3}
              />
            </div>
          )}

          {/* Code Editor */}
          <div>
            <label className="block text-sm font-medium mb-1">Code Editor</label>
            <textarea
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="w-full h-[400px] px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary font-mono text-sm resize-none"
              spellCheck={false}
              onKeyDown={(e) => {
                if (e.ctrlKey && e.key === 'Enter') {
                  e.preventDefault();
                  handleExecute();
                }
              }}
            />
          </div>

          {/* Results */}
          {result && (
            <div className={`p-4 border rounded-lg ${getStatusColor(result.status)}`}>
              <div className="flex items-center justify-between mb-3">
                <span className="font-semibold">{getStatusLabel(result.status)}</span>
                <div className="text-sm space-x-4">
                  <span>Time: {result.execution_time_ms}ms</span>
                  <span>Memory: {result.memory_used_kb} KB</span>
                  {result.exit_code !== null && <span>Exit Code: {result.exit_code}</span>}
                </div>
              </div>

              {result.stdout && (
                <div className="mb-3">
                  <div className="text-xs font-semibold mb-1 uppercase">Output</div>
                  <pre className="bg-white p-3 rounded border overflow-auto max-h-[200px] font-mono text-sm">
                    {result.stdout}
                  </pre>
                </div>
              )}

              {result.stderr && (
                <div className="mb-3">
                  <div className="text-xs font-semibold mb-1 uppercase">Errors</div>
                  <pre className="bg-white p-3 rounded border overflow-auto max-h-[200px] font-mono text-sm text-red-600">
                    {result.stderr}
                  </pre>
                </div>
              )}

              {result.error_message && (
                <div>
                  <div className="text-xs font-semibold mb-1 uppercase">Message</div>
                  <p className="text-sm">{result.error_message}</p>
                </div>
              )}
            </div>
          )}

          {/* Error Alert */}
          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          {/* Tips */}
          <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
            <h4 className="font-semibold text-blue-900 text-sm mb-2">💡 Tips</h4>
            <ul className="text-xs text-blue-800 space-y-1">
              <li>• Code executes in an isolated Docker container with no network access</li>
              <li>• Default timeout: 5 seconds, Memory limit: 128MB</li>
              <li>• Use custom input for programs that read from stdin</li>
              <li>• Press Ctrl+Enter to quickly run your code</li>
            </ul>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default CodeSandbox;
