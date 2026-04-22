import clsx from 'clsx';

interface AvatarProps {
  name: string;
  src?: string;
  size?: 'sm' | 'md' | 'lg';
}

const COLORS = [
  'bg-brand-500', 'bg-accent-400', 'bg-gold-400', 'bg-brand-300',
  'bg-accent-300', 'bg-brand-700', 'bg-gold-500',
];

function getColor(name: string) {
  let hash = 0;
  for (let i = 0; i < name.length; i++) hash = name.charCodeAt(i) + ((hash << 5) - hash);
  return COLORS[Math.abs(hash) % COLORS.length];
}

function getInitials(name: string) {
  return name.split(' ').map(n => n[0]).join('').slice(0, 2).toUpperCase();
}

export function Avatar({ name, src, size = 'md' }: AvatarProps) {
  return (
    <div
      className={clsx(
        'rounded-full flex items-center justify-center text-ink-inverse font-semibold font-[family-name:var(--font-display)] shrink-0',
        getColor(name),
        {
          'w-7 h-7 text-xs': size === 'sm',
          'w-9 h-9 text-sm': size === 'md',
          'w-12 h-12 text-base': size === 'lg',
        }
      )}
    >
      {src ? (
        <img src={src} alt={name} className="w-full h-full rounded-full object-cover" />
      ) : (
        getInitials(name)
      )}
    </div>
  );
}
