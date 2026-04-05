import type { FC } from 'react';
import './styles.scss';
import type { InputProps } from './types';

export const Input: FC<InputProps> = ({
  value,
  placeholder,
  label,
  id,
  disabled,
  onChange,
}) => {
  return (
    <div className="input-wrapper">
      {label && <label htmlFor={id}>{label}</label>}

      <input
        className="input"
        value={value}
        id={id}
        disabled={disabled}
        placeholder={placeholder}
        onChange={(e) => onChange?.(e.target.value)}
      />
    </div>
  );
};
