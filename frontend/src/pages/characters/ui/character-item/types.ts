export type CharacterItemProps = {
  uid: string;
  name: string;
  description: string;
  avatarUid?: string | null;

  onEdit?: (uid: string) => void;
  onDelete?: (uid: string) => void;
};
