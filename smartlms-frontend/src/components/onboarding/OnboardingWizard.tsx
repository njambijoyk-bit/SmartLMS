import React, { useState, useEffect } from 'react';
import { Button } from '../ui/Button';
import { Input } from '../ui/Input';
import { Label } from '../ui/Label';
import { Card, CardHeader, CardTitle, CardContent } from '../ui/Card';
import { Alert, AlertDescription } from '../ui/Alert';

interface InstitutionData {
  name: string;
  subdomain: string;
  email: string;
  country: string;
  plan: 'starter' | 'growth' | 'enterprise';
  useCase: string;
  studentCount: string;
}

interface OnboardingWizardProps {
  onComplete?: (data: InstitutionData) => void;
}

export const OnboardingWizard: React.FC<OnboardingWizardProps> = ({ onComplete }) => {
  const [step, setStep] = useState(1);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  const [formData, setFormData] = useState<InstitutionData>({
    name: '',
    subdomain: '',
    email: '',
    country: '',
    plan: 'starter',
    useCase: '',
    studentCount: '',
  });

  const totalSteps = 4;

  const updateField = (field: keyof InstitutionData, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    setError(null);
  };

  const validateStep = (): boolean => {
    switch (step) {
      case 1:
        if (!formData.name.trim()) {
          setError('Institution name is required');
          return false;
        }
        if (!formData.subdomain.trim()) {
          setError('Subdomain is required');
          return false;
        }
        if (!/^[a-z0-9-]+$/.test(formData.subdomain)) {
          setError('Subdomain can only contain lowercase letters, numbers, and hyphens');
          return false;
        }
        break;
      case 2:
        if (!formData.email.trim()) {
          setError('Email is required');
          return false;
        }
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
          setError('Please enter a valid email address');
          return false;
        }
        if (!formData.country.trim()) {
          setError('Country is required');
          return false;
        }
        break;
      case 3:
        if (!formData.studentCount.trim()) {
          setError('Please select student count');
          return false;
        }
        break;
      case 4:
        if (!formData.useCase.trim()) {
          setError('Please describe your primary use case');
          return false;
        }
        break;
    }
    return true;
  };

  const handleNext = () => {
    if (validateStep()) {
      setStep(prev => Math.min(prev + 1, totalSteps));
    }
  };

  const handleBack = () => {
    setStep(prev => Math.max(prev - 1, 1));
  };

  const handleSubmit = async () => {
    if (!validateStep()) return;

    setIsSubmitting(true);
    setError(null);

    try {
      const response = await fetch('/api/institutions/signup', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData),
      });

      if (!response.ok) {
        const data = await response.json();
        throw new Error(data.error || 'Signup failed');
      }

      const result = await response.json();
      
      if (onComplete) {
        onComplete(formData);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An error occurred');
    } finally {
      setIsSubmitting(false);
    }
  };

  const renderStepIndicator = () => (
    <div className="flex items-center justify-center mb-8">
      {Array.from({ length: totalSteps }, (_, i) => (
        <React.Fragment key={i}>
          <div
            className={`w-10 h-10 rounded-full flex items-center justify-center font-semibold transition-colors ${
              i + 1 <= step
                ? 'bg-primary text-primary-foreground'
                : 'bg-muted text-muted-foreground'
            }`}
          >
            {i + 1}
          </div>
          {i < totalSteps - 1 && (
            <div
              className={`w-16 h-1 mx-2 transition-colors ${
                i + 1 < step ? 'bg-primary' : 'bg-muted'
              }`}
            />
          )}
        </React.Fragment>
      ))}
    </div>
  );

  const renderStep1 = () => (
    <div className="space-y-4">
      <div>
        <Label htmlFor="name">Institution Name</Label>
        <Input
          id="name"
          value={formData.name}
          onChange={(e) => updateField('name', e.target.value)}
          placeholder="e.g., University of Nairobi"
          autoFocus
        />
      </div>
      <div>
        <Label htmlFor="subdomain">Your SmartLMS Subdomain</Label>
        <div className="flex">
          <Input
            id="subdomain"
            value={formData.subdomain}
            onChange={(e) => updateField('subdomain', e.target.value.toLowerCase())}
            placeholder="your-institution"
            className="rounded-r-none"
          />
          <span className="px-4 py-2 bg-muted border border-l-0 rounded-r-md text-muted-foreground">
            .smartlms.com
          </span>
        </div>
        <p className="text-sm text-muted-foreground mt-1">
          This will be your institution's unique URL
        </p>
      </div>
    </div>
  );

  const renderStep2 = () => (
    <div className="space-y-4">
      <div>
        <Label htmlFor="email">Admin Email</Label>
        <Input
          id="email"
          type="email"
          value={formData.email}
          onChange={(e) => updateField('email', e.target.value)}
          placeholder="admin@your-institution.edu"
          autoFocus
        />
      </div>
      <div>
        <Label htmlFor="country">Country</Label>
        <Input
          id="country"
          value={formData.country}
          onChange={(e) => updateField('country', e.target.value)}
          placeholder="e.g., Kenya"
        />
      </div>
    </div>
  );

  const renderStep3 = () => (
    <div className="space-y-4">
      <div>
        <Label>Approximate Student Count</Label>
        <div className="grid grid-cols-2 gap-4 mt-2">
          {[
            { value: '1-100', label: 'Small (1-100)' },
            { value: '101-1000', label: 'Medium (101-1,000)' },
            { value: '1001-10000', label: 'Large (1,001-10,000)' },
            { value: '10001+', label: 'Enterprise (10,000+)' },
          ].map((option) => (
            <button
              key={option.value}
              type="button"
              onClick={() => updateField('studentCount', option.value)}
              className={`p-4 border rounded-lg text-left transition-all ${
                formData.studentCount === option.value
                  ? 'border-primary bg-primary/5 ring-2 ring-primary'
                  : 'hover:border-primary/50'
              }`}
            >
              <div className="font-medium">{option.label}</div>
            </button>
          ))}
        </div>
      </div>
      <div>
        <Label>Recommended Plan</Label>
        <div className="mt-2 p-4 bg-muted rounded-lg">
          <div className="font-semibold mb-2">
            {formData.studentCount === '1-100' && 'Starter Plan'}
            {formData.studentCount === '101-1000' && 'Growth Plan'}
            {formData.studentCount === '1001-10000' && 'Enterprise Plan'}
            {formData.studentCount === '10001+' && 'Enterprise Plan (Contact Sales)'}
            {!formData.studentCount && 'Select student count to see recommendation'}
          </div>
          <p className="text-sm text-muted-foreground">
            {formData.studentCount === '1-100' && 'Perfect for small institutions. Includes all core features.'}
            {formData.studentCount === '101-1000' && 'Ideal for growing institutions with advanced needs.'}
            {formData.studentCount === '1001-10000' && 'Full-featured solution for large institutions.'}
            {formData.studentCount === '10001+' && 'Custom enterprise solution with dedicated support.'}
            {!formData.studentCount && ''}
          </p>
        </div>
      </div>
    </div>
  );

  const renderStep4 = () => (
    <div className="space-y-4">
      <div>
        <Label htmlFor="useCase">Primary Use Case</Label>
        <textarea
          id="useCase"
          value={formData.useCase}
          onChange={(e) => updateField('useCase', e.target.value)}
          placeholder="Tell us about how you plan to use SmartLMS..."
          className="w-full min-h-[120px] px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-primary"
          autoFocus
        />
        <p className="text-sm text-muted-foreground mt-1">
          This helps us customize your onboarding experience
        </p>
      </div>
      <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
        <h4 className="font-semibold text-blue-900 mb-2">What happens next?</h4>
        <ul className="text-sm text-blue-800 space-y-1">
          <li>✓ Your 14-day sandbox will be created instantly</li>
          <li>✓ Sample data will be pre-loaded for exploration</li>
          <li>✓ You'll receive a welcome email with setup guide</li>
          <li>✓ Our team may reach out to offer assistance</li>
        </ul>
      </div>
    </div>
  );

  return (
    <Card className="max-w-2xl mx-auto">
      <CardHeader>
        <CardTitle className="text-2xl text-center">
          Create Your Institution
        </CardTitle>
        <p className="text-muted-foreground text-center">
          Start your 14-day free sandbox - no credit card required
        </p>
      </CardHeader>
      <CardContent>
        {renderStepIndicator()}

        {error && (
          <Alert variant="destructive" className="mb-6">
            <AlertDescription>{error}</AlertDescription>
          </Alert>
        )}

        <div className="min-h-[300px]">
          {step === 1 && renderStep1()}
          {step === 2 && renderStep2()}
          {step === 3 && renderStep3()}
          {step === 4 && renderStep4()}
        </div>

        <div className="flex justify-between mt-8">
          <Button
            variant="outline"
            onClick={handleBack}
            disabled={step === 1 || isSubmitting}
          >
            Back
          </Button>
          
          {step < totalSteps ? (
            <Button onClick={handleNext} disabled={isSubmitting}>
              Next
            </Button>
          ) : (
            <Button 
              onClick={handleSubmit} 
              disabled={isSubmitting}
              className="px-8"
            >
              {isSubmitting ? 'Creating...' : 'Create Sandbox'}
            </Button>
          )}
        </div>

        <p className="text-xs text-muted-foreground text-center mt-6">
          By creating an account, you agree to our{' '}
          <a href="/terms" className="underline hover:text-primary">Terms of Service</a>
          {' '}and{' '}
          <a href="/privacy" className="underline hover:text-primary">Privacy Policy</a>
        </p>
      </CardContent>
    </Card>
  );
};

export default OnboardingWizard;
