import type { WindowProps } from '@shared/ui';

export type PersonaFormWindowMode = 'create' | 'edit';

export type PersonaFormWindowData = {
  mode?: PersonaFormWindowMode;
  uid?: string;
  initialName?: string;
  initialDescription?: string;
};

export type PersonaFormWindowProps = WindowProps & PersonaFormWindowData;
