export type CharacterFormPageProps = {
  mode: 'create' | 'edit';
  uid?: string;
};

export type CharacterFormStateProps = {
  mode: 'create' | 'edit';
  uid?: string;
  initialName: string;
  initialDescription: string;
  avatarUid?: string | null;
};
