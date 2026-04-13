import { Text } from '@shared/ui/text';

import type { InputProps } from './types';
import type { ComponentPropsWithRef, FC } from 'react';

import './styles.scss';

export const Input: FC<InputProps> = ({
  id,
  label,
  onChange,
  multiline,
  ...rest
}) => {
  return (
    <div className="input-wrapper">
      {label && (
        <Text tag="label" htmlFor={id}>
          {label}
        </Text>
      )}

      {multiline ? (
        <textarea
          className="input scrollbar"
          id={id}
          {...(rest as ComponentPropsWithRef<'textarea'>)}
          onChange={(e) => onChange?.(e.target.value)}
        />
      ) : (
        <input
          className="input"
          id={id}
          {...(rest as ComponentPropsWithRef<'input'>)}
          onChange={(e) => onChange?.(e.target.value)}
        />
      )}
    </div>
  );
};
