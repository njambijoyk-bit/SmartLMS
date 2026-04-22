import { forwardRef, type SelectHTMLAttributes } from 'react';
import clsx from 'clsx';

interface SelectProps extends SelectHTMLAttributes<HTMLSelectElement> {
  label?: string;
  error?: string;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(
  ({ label, error, className, children, ...props }, ref) => {
    return (
      <div className="space-y-1.5">
        {label && (
          <label className="block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)]">
            {label}
          </label>
        )}
        <select
          ref={ref}
          className={clsx(
            'w-full rounded-lg border bg-surface-raised px-3.5 py-2.5 text-sm text-ink',
            'focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400',
            'transition-colors duration-150',
            'cursor-pointer',
            error ? 'border-danger' : 'border-sand-300',
            className
          )}
          {...props}
        >
          {children}
        </select>
        {error && <p className="text-xs text-danger">{error}</p>}
      </div>
    );
  }
);

Select.displayName = 'Select';
