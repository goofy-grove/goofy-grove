import type { WindowProps } from '@shared/ui';

export type CharacterFormWindowMode = 'create' | 'edit';

export type CharacterFormWindowData = {
  mode?: CharacterFormWindowMode;
  uid?: string;
  initialName?: string;
  initialDescription?: string;
};

export type CharacterFormWindowProps = WindowProps & CharacterFormWindowData;
