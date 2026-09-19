import type { ButtonProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const Button: FC<ButtonProps> = ({
  children,
  type = 'button',
  rightIcon,
  leftIcon,
  className,
  variant = 'default',
  color = 'default',
  size = 'default',
  ...props
}) => (
  <button
    type={type}
    {...props}
    className={[
      'button',
      size === 'compact' ? 'button--compact' : '',
      variant !== 'default' ? `button--${variant}` : '',
      color !== 'default' ? `button--${color}` : '',
      !children ? 'button--iconic' : '',
      className ?? '',
    ]
      .filter(Boolean)
      .join(' ')}
  >
    {leftIcon && <span className="button__icon">{leftIcon}</span>}

    {children && <span className="button__content">{children}</span>}

    {rightIcon && <span className="button__icon">{rightIcon}</span>}
  </button>
);
