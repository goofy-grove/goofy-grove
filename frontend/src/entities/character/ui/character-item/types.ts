export type CharacterItemProps = {
  uid: string;
  name: string;
  description: string;

  onEdit?: (uid: string) => void;
  onDelete?: (uid: string) => void;
};
