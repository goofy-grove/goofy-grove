import { IconX } from '@tabler/icons-react';

import { Button } from '../button';

import type { FC } from 'react';
import type { CardProps } from './types';

import './styles.scss';

export const Card: FC<CardProps> = ({
  children,
  closable,
  title,
  headerRef,
  className,
  actions,
  onClose,
  ...rest
}) => {
  return (
    <div className={`card ${className ?? ''}`} {...rest}>
      <div className="card__header" ref={headerRef}>
        <div className="card__header__title">{title}</div>

        <div className="card__header__actions">
          {actions}

          {closable && (
            <Button variant="ghost" onClick={onClose} rightIcon={<IconX />} />
          )}
        </div>
      </div>

      <div className="card__content">{children}</div>
    </div>
  );
};
