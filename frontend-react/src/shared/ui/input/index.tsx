import { Text } from '../text';

import type { InputProps } from './types';
import type { FC } from 'react';

import './styles.scss';

export const Input: FC<InputProps> = ({ id, label, onChange, ...rest }) => {
  return (
    <div className="input-wrapper">
      {label && (
        <Text tag="label" htmlFor={id}>
          {label}
        </Text>
      )}

      <input
        className="input"
        id={id}
        {...rest}
        onChange={(e) => onChange?.(e.target.value)}
      />
    </div>
  );
};
