export type SelectItem = {
  value: string;
  label: string;
};

export type SelectItemProps = SelectItem & {
  selected: boolean;

  onClick: (value: string) => void;
};

type SingleSelect = {
  multiselect?: false;
  selected?: string;

  onChange?: (value: string) => void;
};

type MultiSelect = {
  multiselect: true;
  selected?: string[];
  onChange?: (value: string[]) => void;
};

export type SelectState = SingleSelect | MultiSelect;

export type SelectProps = {
  items: SelectItem[];
  placeholder?: string;
} & SelectState;
