import { useState, type FC } from 'react';

import type { AvatarProps } from './types';

import './styles.scss';

export const Avatar: FC<AvatarProps> = ({ className, size, src, ...props }) => {
  const [error, setError] = useState(false);

  return (
    <div className={`avatar ${size ?? 'medium'} ${className ?? ''}`}>
      {!error && src && (
        <img
          className="avatar__image"
          src={src}
          {...props}
          onError={() => setError(true)}
        />
      )}
    </div>
  );
};
