import type { FC } from 'react';
import './styles.scss';
import type { InputProps } from './types';

export const Input: FC<InputProps> = ({ value, placeholder, onChange }) => {
  return (
    <input
      className="input"
      value={value}
      placeholder={placeholder}
      onChange={(e) => onChange?.(e.target.value)}
    />
  );
};
