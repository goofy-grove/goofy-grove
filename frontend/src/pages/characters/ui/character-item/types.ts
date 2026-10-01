export type CharacterItemProps = {
  uid: string;
  name: string;
  description: string;
  avatarUid?: string | null;
  showActions?: boolean;

  onEdit?: (uid: string) => void;
  onDelete?: (uid: string) => void;
  onOpenAction?: (uid: string) => void;
  onClick?: (uid: string) => void;
};
