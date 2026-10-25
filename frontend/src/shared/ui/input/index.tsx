import { IconCheck, IconCopy } from '@tabler/icons-react';
import { useId } from 'react';

import { Button } from '@shared/ui/button';
import { useCopy } from '@shared/ui/hooks';
import { Text } from '@shared/ui/text';

import type { InputProps } from './types';
import type { ComponentPropsWithRef, FC } from 'react';

import './styles.scss';

export const Input: FC<InputProps> = ({
  id,
  label,
  hint,
  'aria-describedby': describedBy,
  multiline,
  className,
  isCopyable,
  onChange,
  value,
  ...rest
}) => {
  const generatedId = useId();
  const inputId = id ?? generatedId;
  const hintId = `${inputId}-hint`;

  const descriptionIds =
    [describedBy, hint ? hintId : undefined].filter(Boolean).join(' ') ||
    undefined;

  const { isCopied, handleCopy } = useCopy(value?.toString() || '');

  return (
    <div className="input-wrapper">
      {label && (
        <Text tag="label" htmlFor={inputId}>
          {label}
        </Text>
      )}

      <div className="input-with-actions">
        {multiline ? (
          <textarea
            className={`input scrollbar ${className}`}
            id={inputId}
            aria-describedby={descriptionIds}
            value={value}
            {...(rest as ComponentPropsWithRef<'textarea'>)}
            onChange={(e) => onChange?.(e.target.value)}
          />
        ) : (
          <input
            className={`input ${className}`}
            id={inputId}
            aria-describedby={descriptionIds}
            value={value}
            {...(rest as ComponentPropsWithRef<'input'>)}
            onChange={(e) => onChange?.(e.target.value)}
          />
        )}

        {isCopyable && (
          <Button
            className="input__copy-button"
            variant="ghost"
            color={isCopied ? 'success' : 'default'}
            leftIcon={isCopied ? <IconCheck /> : <IconCopy />}
            onClick={!isCopied ? handleCopy : () => {}}
          />
        )}
      </div>

      {hint && (
        <p className="input-hint" id={hintId}>
          {hint}
        </p>
      )}
    </div>
  );
};
