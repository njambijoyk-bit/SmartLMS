import { forwardRef, type LabelHTMLAttributes } from 'react';
import clsx from 'clsx';

interface LabelProps extends LabelHTMLAttributes<HTMLLabelElement> {
  error?: string;
}

export const Label = forwardRef<HTMLLabelElement, LabelProps>(
  ({ className, children, error, ...props }, ref) => {
    return (
      <label
        ref={ref}
        className={clsx(
          'block text-sm font-medium text-ink-secondary font-[family-name:var(--font-display)]',
          error && 'text-danger',
          className
        )}
        {...props}
      >
        {children}
      </label>
    );
  }
);

Label.displayName = 'Label';
