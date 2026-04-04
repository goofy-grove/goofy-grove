import { IconX } from '@tabler/icons-react';

import { Button } from '../button';

import type { FC } from 'react';
import type { CardProps } from './types';

import './styles.scss';

export const Card: FC<CardProps> = ({
  children,
  closable,
  onClose,
  className,
  ...rest
}) => {
  return (
    <div className={`card ${className ?? ''}`} {...rest}>
      <div className="card__header">
        <div className="card__header__title">Card title</div>

        {closable && (
          <Button
            variant="ghost"
            onClick={onClose}
            rightIcon={<IconX />}
          ></Button>
        )}
      </div>

      <div className="card__content">{children}</div>
    </div>
  );
};
