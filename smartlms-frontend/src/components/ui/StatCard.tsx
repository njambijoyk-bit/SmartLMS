import { type ReactNode } from 'react';
import { TrendingUp, TrendingDown, Minus } from 'lucide-react';
import clsx from 'clsx';

interface StatCardProps {
  label: string;
  value: string | number;
  change?: number;
  trend?: 'up' | 'down' | 'flat';
  icon: ReactNode;
  accentColor?: string;
}

export function StatCard({ label, value, change, trend, icon, accentColor }: StatCardProps) {
  return (
    <div className="bg-surface-raised rounded-xl border border-sand-200 p-5 relative overflow-hidden group hover:border-brand-200 transition-colors">
      <div className={clsx(
        'absolute top-0 right-0 w-20 h-20 rounded-bl-[40px] opacity-[0.07] transition-opacity group-hover:opacity-[0.12]',
        accentColor || 'bg-brand-500'
      )} />
      <div className="flex items-start justify-between mb-3">
        <div className={clsx('p-2 rounded-lg', accentColor ? 'bg-sand-100' : 'bg-brand-50')}>
          {icon}
        </div>
        {change !== undefined && (
          <div className={clsx('flex items-center gap-0.5 text-xs font-medium', {
            'text-success': trend === 'up',
            'text-danger': trend === 'down',
            'text-ink-tertiary': trend === 'flat',
          })}>
            {trend === 'up' && <TrendingUp size={12} />}
            {trend === 'down' && <TrendingDown size={12} />}
            {trend === 'flat' && <Minus size={12} />}
            {change > 0 ? '+' : ''}{change}%
          </div>
        )}
      </div>
      <div className="text-2xl font-bold font-[family-name:var(--font-display)] text-ink tracking-tight">{value}</div>
      <div className="text-sm text-ink-tertiary mt-0.5">{label}</div>
    </div>
  );
}
