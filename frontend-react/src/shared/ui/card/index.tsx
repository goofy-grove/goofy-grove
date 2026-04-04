import { IconX } from '@tabler/icons-react';

import { Button } from '../button';

import type { FC } from 'react';
import type { CardProps } from './types';

import './styles.scss';

export const Card: FC<CardProps> = ({
  children,
  closable,
  headerRef,
  className,
  onClose,
  ...rest
}) => {
  return (
    <div className={`card ${className ?? ''}`} {...rest}>
      <div className="card__header" ref={headerRef}>
        <div className="card__header__title">Card title</div>

        {closable && (
          <Button variant="ghost" onClick={onClose} rightIcon={<IconX />} />
        )}
      </div>

      <div className="card__content">{children}</div>
    </div>
  );
};
