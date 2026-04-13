import type { ComponentPropsWithRef } from 'react';

type InputBaseProps = {
  label?: string;
  onChange?: (value: string) => void;
};

type InputMultilineProps = Omit<
  ComponentPropsWithRef<'textarea'>,
  'onInput' | 'onChange'
> & {
  multiline: true;
};

type InputSinglelineProps = Omit<
  ComponentPropsWithRef<'input'>,
  'onInput' | 'onChange'
> & {
  multiline?: false;
};

export type InputProps = InputBaseProps &
  (InputMultilineProps | InputSinglelineProps);
