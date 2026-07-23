export type CharacterEventData = {
  uid: string;
  name: string;
  description: string;
  creator_uid: string;
  avatar_uid?: string | null;
};

export type CharacterDeletedEventData = {
  uid: string;
};
