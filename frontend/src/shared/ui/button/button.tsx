import type { ButtonProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const Button: FC<ButtonProps> = ({
  children,
  rightIcon,
  leftIcon,
  className,
  variant = 'default',
  color = 'default',
  disabled,
  onClick,
}) => (
  <button
    className={[
      'button',
      variant !== 'default' ? `button--${variant}` : '',
      color !== 'default' ? `button--${color}` : '',
      !children ? 'button--iconic' : '',
      className ?? '',
    ]
      .filter(Boolean)
      .join(' ')}
    disabled={disabled}
    onClick={onClick}
  >
    {leftIcon && <div className="button__icon">{leftIcon}</div>}

    {children && <div className="button__content">{children}</div>}

    {rightIcon && <div className="button__icon">{rightIcon}</div>}
  </button>
);
