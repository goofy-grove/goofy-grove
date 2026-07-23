import { type FC, useState } from 'react';

import type { AvatarProps } from './types';

import './styles.scss';

export const Avatar: FC<AvatarProps> = ({ className, size, src, ...props }) => {
  const [failedSrc, setFailedSrc] = useState<string | undefined>();
  const showImage = Boolean(src) && failedSrc !== src;

  return (
    <div className={`avatar ${size ?? 'medium'} ${className ?? ''}`}>
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
