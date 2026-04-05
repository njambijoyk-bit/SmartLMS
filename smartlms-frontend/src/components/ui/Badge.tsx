import clsx from 'clsx';

interface BadgeProps {
  children: React.ReactNode;
  variant?: 'default' | 'success' | 'warning' | 'danger' | 'info' | 'brand' | 'accent';
  size?: 'sm' | 'md';
}

export function Badge({ children, variant = 'default', size = 'sm' }: BadgeProps) {
  return (
    <span
      className={clsx(
        'inline-flex items-center rounded-full font-medium font-[family-name:var(--font-display)]',
        {
          'bg-sand-200 text-ink-secondary': variant === 'default',
          'bg-success-light text-success': variant === 'success',
          'bg-warning-light text-warning': variant === 'warning',
          'bg-danger-light text-danger': variant === 'danger',
          'bg-info-light text-info': variant === 'info',
          'bg-brand-50 text-brand-600': variant === 'brand',
          'bg-accent-50 text-accent-500': variant === 'accent',
          'px-2 py-0.5 text-xs': size === 'sm',
          'px-2.5 py-1 text-sm': size === 'md',
        }
      )}
    >
      {children}
    </span>
  );
}
