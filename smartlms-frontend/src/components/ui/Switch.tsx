import { forwardRef, type InputHTMLAttributes } from 'react';
import clsx from 'clsx';

interface SwitchProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type' | 'size'> {
  label?: string;
}

export const Switch = forwardRef<HTMLInputElement, SwitchProps>(
  ({ label, className, ...props }, ref) => {
    return (
      <label className="inline-flex items-center cursor-pointer">
        <div className="relative">
          <input
            type="checkbox"
            ref={ref}
            className="sr-only peer"
            {...props}
          />
          <div
            className={clsx(
              'w-10 h-6 rounded-full bg-sand-300',
              'peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-brand-300',
              'peer-checked:bg-brand-500',
              'transition-colors duration-200',
              className
            )}
          />
          <div
            className={clsx(
              'absolute left-1 top-1 w-4 h-4 rounded-full bg-white',
              'peer-checked:translate-x-4',
              'transition-transform duration-200'
            )}
          />
        </div>
        {label && <span className="ml-2 text-sm text-ink-secondary">{label}</span>}
      </label>
    );
  }
);

Switch.displayName = 'Switch';
