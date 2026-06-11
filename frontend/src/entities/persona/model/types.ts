export type PersonaEventData = {
  uid: string;
  name: string;
  description: string;
  creator_uid: string;
};

export type PersonaDeletedEventData = {
  uid: string;
};
