import type { ComponentPropsWithoutRef } from 'react';

export type InputProps = Omit<
  ComponentPropsWithoutRef<'input'>,
  'onChange' | 'onInput'
> & {
  value?: string;

  placeholder?: string;
  label?: string;
  id?: string;

  disabled?: boolean;

  onChange?: (value: string) => void;
};
