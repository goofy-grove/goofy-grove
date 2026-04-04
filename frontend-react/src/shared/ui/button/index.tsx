import type { FC } from 'react';
import type { ButtonProps } from './types';

import './styles.scss';

export const Button: FC<ButtonProps> = ({
  children,
  rightIcon,
  leftIcon,
  variant,
  onClick,
}) => (
  <button
    className={`button ${variant ?? ''} ${!children ? 'iconic' : ''}`}
    onClick={onClick}
  >
    {leftIcon && <div className="button__icon">{leftIcon}</div>}

    {children && <div className="button__content">{children}</div>}

    {rightIcon && <div className="button__icon">{rightIcon}</div>}
  </button>
);
