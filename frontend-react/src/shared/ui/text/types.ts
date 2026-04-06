import type { ComponentPropsWithoutRef, ElementType, ReactNode } from 'react';

type TextPropsWith<T extends ElementType> = ComponentPropsWithoutRef<T> & {
  tag?: T;
};

type TextOwnProps =
  | TextPropsWith<'p'>
  | TextPropsWith<'span'>
  | TextPropsWith<'label'>
  | TextPropsWith<'h1'>
  | TextPropsWith<'h2'>
  | TextPropsWith<'h3'>
  | TextPropsWith<'h4'>
  | TextPropsWith<'h5'>
  | TextPropsWith<'h6'>;

export type TextProps = {
  children: ReactNode;
  variant?:
    | 'primary'
    | 'secondary'
    | 'tertiary'
    | 'link'
    | 'error'
    | 'success'
    | 'warning'
    | 'accent';
} & TextOwnProps;
