export type CharacterItemProps = {
  uid: string;
  name: string;
  description: string;
  avatarUid?: string | null;
  showActions?: boolean;

  onEdit?: (uid: string) => void;
  onDelete?: (uid: string) => void;
  onLongPress?: (uid: string) => void;
  onClick?: (uid: string) => void;
};
