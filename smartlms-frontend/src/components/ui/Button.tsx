import { type ButtonHTMLAttributes, forwardRef } from 'react';
import clsx from 'clsx';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'danger' | 'accent' | 'outline';
  size?: 'sm' | 'md' | 'lg';
  loading?: boolean;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ variant = 'primary', size = 'md', loading, className, children, disabled, ...props }, ref) => {
    return (
      <button
        ref={ref}
        disabled={disabled || loading}
        className={clsx(
          'inline-flex items-center justify-center gap-2 rounded-lg font-medium font-[family-name:var(--font-display)] transition-all duration-200 cursor-pointer',
          'disabled:opacity-50 disabled:cursor-not-allowed',
          {
            'bg-brand-500 text-ink-inverse hover:bg-brand-600 active:bg-brand-700 shadow-sm hover:shadow': variant === 'primary',
            'bg-surface-raised text-ink border border-sand-300 hover:border-brand-300 hover:text-brand-600 active:bg-sand-100': variant === 'secondary',
            'text-ink-secondary hover:text-ink hover:bg-sand-100 active:bg-sand-200': variant === 'ghost',
            'bg-danger text-ink-inverse hover:bg-red-700 active:bg-red-800': variant === 'danger',
            'bg-accent-400 text-ink-inverse hover:bg-accent-500 active:bg-accent-600 shadow-sm': variant === 'accent',
            'bg-transparent text-ink-secondary border border-sand-300 hover:border-brand-300 hover:text-brand-600 active:bg-sand-50': variant === 'outline',
            'px-3 py-1.5 text-sm': size === 'sm',
            'px-4 py-2.5 text-sm': size === 'md',
            'px-6 py-3 text-base': size === 'lg',
          },
          className
        )}
        {...props}
      >
        {loading && (
          <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24" fill="none">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" />
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        )}
        {children}
      </button>
    );
  }
);

Button.displayName = 'Button';
