import { useId } from 'react';

import { Text } from '@shared/ui/text';

import type { InputProps } from './types';
import type { ComponentPropsWithRef, FC } from 'react';

import './styles.scss';

export const Input: FC<InputProps> = ({
  id,
  label,
  hint,
  'aria-describedby': describedBy,
  onChange,
  multiline,
  ...rest
}) => {
  const generatedId = useId();
  const inputId = id ?? generatedId;
  const hintId = `${inputId}-hint`;
  const descriptionIds =
    [describedBy, hint ? hintId : undefined].filter(Boolean).join(' ') ||
    undefined;

  return (
    <div className="input-wrapper">
      {label && (
        <Text tag="label" htmlFor={inputId}>
          {label}
        </Text>
      )}

      {multiline ? (
        <textarea
          className="input scrollbar"
          id={inputId}
          aria-describedby={descriptionIds}
          {...(rest as ComponentPropsWithRef<'textarea'>)}
          onChange={(e) => onChange?.(e.target.value)}
        />
      ) : (
        <input
          className="input"
          id={inputId}
          aria-describedby={descriptionIds}
          {...(rest as ComponentPropsWithRef<'input'>)}
          onChange={(e) => onChange?.(e.target.value)}
        />
      )}

      {hint && (
        <p className="input-hint" id={hintId}>
          {hint}
        </p>
      )}
    </div>
  );
};
