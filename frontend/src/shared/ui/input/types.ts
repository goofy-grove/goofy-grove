import type { ComponentPropsWithRef } from 'react';

export type InputProps = Omit<
  ComponentPropsWithRef<'input'>,
  'onChange' | 'onInput'
> & {
  value?: string;

  placeholder?: string;
  label?: string;
  id?: string;

  disabled?: boolean;

  onChange?: (value: string) => void;
};
