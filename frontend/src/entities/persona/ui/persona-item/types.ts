export type PersonaItemProps = {
  uid: string;
  name: string;
  description: string;

  onEdit?: (uid: string) => void;
  onDelete?: (uid: string) => void;
};
