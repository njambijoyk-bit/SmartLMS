import React, { useState } from 'react';
import { Button } from '../ui/Button';
import { Input } from '../ui/Input';
import { Label } from '../ui/Label';
import { Card, CardHeader, CardTitle, CardContent } from '../ui/Card';
import { Alert, AlertDescription } from '../ui/Alert';

interface FeePaymentProps {
  studentId: string;
  institutionId: string;
  onSuccess?: (paymentId: string) => void;
}

interface PaymentMethod {
  id: string;
  name: string;
  icon: string;
  description: string;
}

const PAYMENT_METHODS: PaymentMethod[] = [
  {
    id: 'mpesa',
    name: 'M-Pesa',
    icon: '📱',
    description: 'Pay via M-Pesa mobile money',
  },
  {
    id: 'card',
    name: 'Credit/Debit Card',
    icon: '💳',
    description: 'Visa, Mastercard, American Express',
  },
  {
    id: 'bank',
    name: 'Bank Transfer',
    icon: '🏦',
    description: 'Direct bank transfer',
  },
  {
    id: 'paypal',
    name: 'PayPal',
    icon: '🅿️',
    description: 'Pay with PayPal account',
  },
];

export const FeePayment: React.FC<FeePaymentProps> = ({
  studentId,
  institutionId,
  onSuccess,
}) => {
  const [selectedMethod, setSelectedMethod] = useState<string>('mpesa');
  const [amount, setAmount] = useState<string>('');
  const [phoneNumber, setPhoneNumber] = useState<string>('');
  const [email, setEmail] = useState<string>('');
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  const handlePayment = async () => {
    if (!amount || parseFloat(amount) <= 0) {
      setError('Please enter a valid amount');
      return;
    }

    if (selectedMethod === 'mpesa' && !phoneNumber) {
      setError('Please enter your M-Pesa phone number');
      return;
    }

    setIsProcessing(true);
    setError(null);

    try {
      const response = await fetch('/api/fees/pay', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          student_id: studentId,
          institution_id: institutionId,
          amount: parseFloat(amount),
          payment_method: selectedMethod,
          phone_number: phoneNumber,
          email: email,
          currency: 'KES',
        }),
      });

      if (!response.ok) {
        const data = await response.json();
        throw new Error(data.error || 'Payment failed');
      }

      const result = await response.json();
      setSuccess(`Payment initiated successfully! ${result.message || ''}`);
      
      if (onSuccess) {
        onSuccess(result.payment_id);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Payment failed');
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <Card className="max-w-2xl mx-auto">
      <CardHeader>
        <CardTitle>Pay School Fees</CardTitle>
        <p className="text-muted-foreground">
          Secure payment via M-Pesa and other methods
        </p>
      </CardHeader>
      <CardContent>
        <div className="space-y-6">
          {/* Amount */}
          <div>
            <Label htmlFor="amount">Amount (KES)</Label>
            <Input
              id="amount"
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="Enter amount to pay"
              min="1"
              step="0.01"
            />
          </div>

          {/* Payment Methods */}
          <div>
            <Label>Select Payment Method</Label>
            <div className="grid grid-cols-2 gap-4 mt-2">
              {PAYMENT_METHODS.map((method) => (
                <button
                  key={method.id}
                  type="button"
                  onClick={() => setSelectedMethod(method.id)}
                  className={`p-4 border rounded-lg text-left transition-all ${
                    selectedMethod === method.id
                      ? 'border-primary bg-primary/5 ring-2 ring-primary'
                      : 'hover:border-primary/50'
                  }`}
                >
                  <div className="text-2xl mb-2">{method.icon}</div>
                  <div className="font-medium">{method.name}</div>
                  <div className="text-xs text-muted-foreground">
                    {method.description}
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* M-Pesa Details */}
          {selectedMethod === 'mpesa' && (
            <div className="p-4 bg-green-50 border border-green-200 rounded-lg space-y-4">
              <div>
                <Label htmlFor="phone">M-Pesa Phone Number</Label>
                <Input
                  id="phone"
                  type="tel"
                  value={phoneNumber}
                  onChange={(e) => setPhoneNumber(e.target.value)}
                  placeholder="e.g., 0712345678"
                />
                <p className="text-xs text-muted-foreground mt-1">
                  You'll receive an M-Pesa prompt on this number
                </p>
              </div>
              <div className="text-sm text-green-800">
                <strong>How it works:</strong>
                <ol className="list-decimal list-inside mt-2 space-y-1">
                  <li>Enter your phone number above</li>
                  <li>Click "Pay Now"</li>
                  <li>Accept the M-Pesa prompt on your phone</li>
                  <li>Enter your M-Pesa PIN</li>
                  <li>Payment confirmation will be instant</li>
                </ol>
              </div>
            </div>
          )}

          {/* Email for Receipt */}
          <div>
            <Label htmlFor="email">Email for Receipt</Label>
            <Input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder="receipt@example.com"
            />
          </div>

          {/* Summary */}
          {amount && (
            <div className="p-4 bg-muted rounded-lg">
              <h4 className="font-semibold mb-2">Payment Summary</h4>
              <div className="space-y-1 text-sm">
                <div className="flex justify-between">
                  <span>Amount:</span>
                  <span>KES {parseFloat(amount).toFixed(2)}</span>
                </div>
                <div className="flex justify-between">
                  <span>Method:</span>
                  <span>{PAYMENT_METHODS.find(m => m.id === selectedMethod)?.name}</span>
                </div>
                {selectedMethod === 'mpesa' && (
                  <div className="flex justify-between">
                    <span>M-Pesa Charges:</span>
                    <span>KES {(parseFloat(amount) * 0.01).toFixed(2)}</span>
                  </div>
                )}
                <div className="flex justify-between font-semibold pt-2 border-t">
                  <span>Total:</span>
                  <span>
                    KES {(parseFloat(amount) + (selectedMethod === 'mpesa' ? parseFloat(amount) * 0.01 : 0)).toFixed(2)}
                  </span>
                </div>
              </div>
            </div>
          )}

          {/* Alerts */}
          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          {success && (
            <Alert className="bg-green-50 border-green-200">
              <AlertDescription className="text-green-800">
                {success}
              </AlertDescription>
            </Alert>
          )}

          {/* Submit Button */}
          <Button
            onClick={handlePayment}
            disabled={isProcessing || !amount}
            className="w-full"
            size="lg"
          >
            {isProcessing ? 'Processing...' : `Pay with ${PAYMENT_METHODS.find(m => m.id === selectedMethod)?.name}`}
          </Button>

          {/* Security Notice */}
          <div className="text-xs text-muted-foreground text-center">
            🔒 Your payment is secured with 256-bit encryption. We never store your payment details.
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default FeePayment;
