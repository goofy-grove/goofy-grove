export type CharacterEventData = {
  id: string;
  name: string;
  description: string;
  creator_uid: string;
};

export type CharacterDeletedEventData = {
  id: string;
};
