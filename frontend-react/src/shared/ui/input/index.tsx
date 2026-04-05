import type { FC } from 'react';
import './styles.scss';
import type { InputProps } from './types';
import { Text } from '../text';

export const Input: FC<InputProps> = ({ id, label, onChange, ...rest }) => {
  return (
    <div className="input-wrapper">
      {label && <Text tag="label" htmlFor={id}>{label}</Text>}

      <input
        className="input"
        id={id}
        {...rest}
        onChange={(e) => onChange?.(e.target.value)}
      />
    </div>
  );
};
