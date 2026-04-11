import { forwardRef, type TextareaHTMLAttributes } from 'react';
import clsx from 'clsx';

interface TextareaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
}

export const Textarea = forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({ label, error, className, ...props }, ref) => {
    return (
      <div className="space-y-1.5">
        {label && (
          <label className="block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)]">
            {label}
          </label>
        )}
        <textarea
          ref={ref}
          className={clsx(
            'w-full rounded-lg border bg-surface-raised px-3.5 py-2.5 text-sm text-ink',
            'placeholder:text-ink-placeholder',
            'focus:outline-none focus:ring-2 focus:ring-brand-300 focus:border-brand-400',
            'transition-colors duration-150',
            'resize-y min-h-[80px]',
            error ? 'border-danger' : 'border-sand-300',
            className
          )}
          {...props}
        />
        {error && <p className="text-xs text-danger">{error}</p>}
      </div>
    );
  }
);

Textarea.displayName = 'Textarea';
