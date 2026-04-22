export type PersonaItemProps = {
  uid: string;
  name: string;
  description: string;

  onEdit?: (uid: string) => void;
};
