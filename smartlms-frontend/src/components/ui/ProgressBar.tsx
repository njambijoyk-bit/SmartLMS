import clsx from 'clsx';

interface ProgressBarProps {
  value: number;
  max?: number;
  size?: 'sm' | 'md';
  color?: 'brand' | 'accent' | 'success' | 'warning' | 'danger';
  showLabel?: boolean;
}

export function ProgressBar({ value, max = 100, size = 'sm', color = 'brand', showLabel = false }: ProgressBarProps) {
  const pct = Math.min(100, Math.max(0, (value / max) * 100));
  return (
    <div className="flex items-center gap-2">
      <div className={clsx('flex-1 rounded-full bg-sand-200 overflow-hidden', {
        'h-1.5': size === 'sm',
        'h-2.5': size === 'md',
      })}>
        <div
          className={clsx('h-full rounded-full transition-all duration-500', {
            'bg-brand-400': color === 'brand',
            'bg-accent-400': color === 'accent',
            'bg-success': color === 'success',
            'bg-warning': color === 'warning',
            'bg-danger': color === 'danger',
          })}
          style={{ width: `${pct}%` }}
        />
      </div>
      {showLabel && <span className="text-xs text-ink-tertiary font-medium tabular-nums">{Math.round(pct)}%</span>}
    </div>
  );
}
