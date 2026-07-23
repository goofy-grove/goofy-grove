export type PersonaEventData = {
  uid: string;
  name: string;
  description: string;
  creator_uid: string;
  avatar_uid?: string | null;
};

export type PersonaDeletedEventData = {
  uid: string;
};
