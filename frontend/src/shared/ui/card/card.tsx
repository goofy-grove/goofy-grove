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
  id,
  withoutHeader,
  onClose,
  ...rest
}) => {
  return (
    <div id={id} className={`card ${className ?? ''}`} {...rest}>
      {!withoutHeader && (
        <div
          className="card__header"
          id={id ? `card-header-${id}` : ''}
          ref={headerRef}
        >
          <div className="card__header__title">{title}</div>

          <div className="card__header__actions">
            {actions}

            {closable && (
              <Button variant="ghost" onClick={onClose} rightIcon={<IconX />} />
            )}
          </div>
        </div>
      )}

      <div className="card__content">{children}</div>
    </div>
  );
};
