import type { ComponentPropsWithoutRef, ElementType, ReactNode } from 'react';

type TextPropsWith<T extends ElementType> = ComponentPropsWithoutRef<T> & {
  tag?: T;
};

type TextOwnProps =
  | TextPropsWith<'p'>
  | TextPropsWith<'span'>
  | TextPropsWith<'label'>;

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
