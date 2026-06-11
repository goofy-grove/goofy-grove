export type CharacterEventData = {
  uid: string;
  name: string;
  description: string;
  creator_uid: string;
};

export type CharacterDeletedEventData = {
  uid: string;
};
