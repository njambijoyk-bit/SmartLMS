import type { ReactNode, HTMLAttributes } from 'react';
import clsx from 'clsx';

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  children: ReactNode;
  padding?: 'none' | 'sm' | 'md' | 'lg';
  hover?: boolean;
}

export function Card({ children, padding = 'md', hover = false, className, ...props }: CardProps) {
  return (
    <div
      className={clsx(
        'bg-surface-raised rounded-xl border border-sand-200',
        hover && 'hover:border-brand-200 hover:shadow-md transition-all duration-200 cursor-pointer',
        {
          '': padding === 'none',
          'p-4': padding === 'sm',
          'p-6': padding === 'md',
          'p-8': padding === 'lg',
        },
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
}
