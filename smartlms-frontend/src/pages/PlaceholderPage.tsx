import { useLocation } from 'react-router-dom';
import { Construction } from 'lucide-react';
import { Card } from '../components/ui/Card';
import { Badge } from '../components/ui/Badge';

export function PlaceholderPage() {
  const location = useLocation();
  const name = location.pathname.slice(1).replace(/-/g, ' ').replace(/^\w/, c => c.toUpperCase());

  return (
    <div className="flex items-center justify-center min-h-[60vh]">
      <Card className="text-center max-w-md w-full">
        <div className="w-14 h-14 rounded-full bg-sand-100 flex items-center justify-center mx-auto mb-4">
          <Construction size={24} className="text-ink-tertiary" />
        </div>
        <h2 className="text-lg font-bold font-[family-name:var(--font-display)] text-ink mb-1">{name}</h2>
        <p className="text-sm text-ink-tertiary mb-4">
          This module is part of the SmartLMS engine and will be built in a future phase.
        </p>
        <Badge variant="brand" size="md">Coming Soon</Badge>
      </Card>
    </div>
  );
}
