import { IconUser } from '@tabler/icons-react';
import { type FC, useState } from 'react';

import type { AvatarProps } from './types';

import './styles.scss';

export const Avatar: FC<AvatarProps> = ({
  className,
  fallback = <IconUser size={32} stroke={1.5} aria-hidden="true" />,
  size,
  src,
  variant,
  ...props
}) => {
  const [failedSrc, setFailedSrc] = useState<string | undefined>();
  const showImage = Boolean(src) && failedSrc !== src;

  const resolvedSize = size ?? (variant !== 'unbordered' ? 'medium' : '');

  return (
    <span
      className={`avatar ${resolvedSize} ${variant ?? ''} ${className ?? ''}`}
    >
      {!showImage && fallback}

      {showImage && src && (
        <img
          className="avatar__image"
          src={src}
          {...props}
          onError={() => setFailedSrc(src)}
        />
      )}
    </span>
  );
};
