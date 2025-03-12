import { useState } from 'react';
import { LuWallet } from 'react-icons/lu';

interface ChainIconProps {
  chain: string;
  iconUrl: string;
  size?: 'sm' | 'md';
  className?: string;
}

export function ChainIcon({ chain, iconUrl, size = 'sm', className = '' }: ChainIconProps) {
  const [showFallback, setShowFallback] = useState(false);
  
  const iconSize = size === 'sm' ? 16 : 20;
  const containerClasses = `text-gray-400 ${size === 'sm' ? 'h-4 w-4' : 'h-5 w-5'} ${className}`;

  if (showFallback) {
    return (
      <span className={containerClasses}>
        <LuWallet size={iconSize} />
      </span>
    );
  }

  return (
    <img 
      src={iconUrl}
      alt={`${chain} icon`}
      className={containerClasses}
      onError={() => setShowFallback(true)}
    />
  );
} 