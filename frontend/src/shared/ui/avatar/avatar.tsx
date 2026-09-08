import { type FC, useState } from 'react';

import type { AvatarProps } from './types';

import './styles.scss';

export const Avatar: FC<AvatarProps> = ({
  className,
  size,
  src,
  variant,
  ...props
}) => {
  const [failedSrc, setFailedSrc] = useState<string | undefined>();
  const showImage = Boolean(src) && failedSrc !== src;

  const resolvedSize = (size ?? variant !== 'unbordered') ? 'medium' : '';

  return (
    <div
      className={`avatar ${resolvedSize} ${variant ?? ''} ${className ?? ''}`}
    >
      {showImage && src && (
        <img
          className="avatar__image"
          src={src}
          {...props}
          onError={() => setFailedSrc(src)}
        />
      )}
    </div>
  );
};
