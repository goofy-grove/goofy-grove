import type { WindowProps } from '../../../../shared/ui';

export type PersonaListWindowProps = Pick<
  WindowProps,
  'onClose' | 'onMaximize'
>;
