import { createContext, useContext, useState, type ReactNode } from 'react';
import { X } from 'lucide-react';
import clsx from 'clsx';

interface DialogContextType {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const DialogContext = createContext<DialogContextType | undefined>(undefined);

interface DialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  children: ReactNode;
}

export function Dialog({ open, onOpenChange, children }: DialogProps) {
  return (
    <DialogContext.Provider value={{ open, onOpenChange }}>
      {children}
    </DialogContext.Provider>
  );
}

interface DialogTriggerProps {
  asChild?: React.ReactElement;
  children: ReactNode;
}

export function DialogTrigger({ asChild, children }: DialogTriggerProps) {
  const context = useContext(DialogContext);
  if (!context) throw new Error('DialogTrigger must be used within Dialog');

  if (asChild) {
    return cloneElement(asChild, {
      onClick: () => context.onOpenChange(true),
    });
  }

  return (
    <button onClick={() => context.onOpenChange(true)} className="cursor-pointer">
      {children}
    </button>
  );
}

interface DialogContentProps {
  children: ReactNode;
  className?: string;
}

export function DialogContent({ children, className }: DialogContentProps) {
  const context = useContext(DialogContext);
  if (!context || !context.open) return null;

  return (
    <>
      <div
        className="fixed inset-0 bg-black/50 backdrop-blur-sm z-50"
        onClick={() => context.onOpenChange(false)}
      />
      <div className="fixed inset-0 flex items-center justify-center z-50 p-4">
        <div
          className={clsx(
            'bg-surface rounded-2xl shadow-xl max-w-lg w-full max-h-[90vh] overflow-y-auto',
            'border border-sand-200',
            className
          )}
        >
          <button
            onClick={() => context.onOpenChange(false)}
            className="absolute top-4 right-4 p-1.5 rounded-lg hover:bg-sand-100 text-ink-tertiary cursor-pointer"
          >
            <X size={18} />
          </button>
          {children}
        </div>
      </div>
    </>
  );
}

interface DialogHeaderProps {
  children: ReactNode;
  className?: string;
}

export function DialogHeader({ children, className }: DialogHeaderProps) {
  return (
    <div className={clsx('p-6 pb-4 border-b border-sand-200', className)}>
      {children}
    </div>
  );
}

interface DialogTitleProps {
  children: ReactNode;
  className?: string;
}

export function DialogTitle({ children, className }: DialogTitleProps) {
  return (
    <h2
      className={clsx(
        'text-lg font-bold font-[family-name:var(--font-display)] text-ink',
        className
      )}
    >
      {children}
    </h2>
  );
}
