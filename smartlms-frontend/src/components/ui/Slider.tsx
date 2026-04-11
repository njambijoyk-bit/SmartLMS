import { forwardRef, type InputHTMLAttributes } from 'react';
import clsx from 'clsx';

interface SliderProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  value?: number[];
  onValueChange?: (value: number[]) => void;
  min?: number;
  max?: number;
  step?: number;
}

export const Slider = forwardRef<HTMLInputElement, SliderProps>(
  ({ value = [0], onValueChange, min = 0, max = 100, step = 1, className }, ref) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      if (onValueChange) {
        onValueChange([parseInt(e.target.value)]);
      }
    };

    const percentage = ((value[0] - min) / (max - min)) * 100;

    return (
      <div className={clsx('relative w-full h-2', className)}>
        <div className="absolute inset-0 bg-sand-200 rounded-full" />
        <div
          className="absolute h-full bg-brand-500 rounded-full"
          style={{ width: `${percentage}%` }}
        />
        <input
          type="range"
          ref={ref}
          min={min}
          max={max}
          step={step}
          value={value[0]}
          onChange={handleChange}
          className="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
        />
        <div
          className="absolute top-1/2 -translate-y-1/2 w-4 h-4 bg-white border-2 border-brand-500 rounded-full shadow-md pointer-events-none"
          style={{ left: `calc(${percentage}% - 8px)` }}
        />
      </div>
    );
  }
);

Slider.displayName = 'Slider';
