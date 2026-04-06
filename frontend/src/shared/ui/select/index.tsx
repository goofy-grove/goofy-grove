import './styles.scss';

import { IconCheck, IconChevronDown } from '@tabler/icons-react';
import { useState, type FC } from 'react';

import { Button } from '../button';
import { Dropdown } from '../dropdown';

import type { SelectItem, SelectItemProps, SelectProps } from './types';

const SelectItem: FC<SelectItemProps> = ({
  value,
  label,
  selected,
  onClick,
}) => {
  return (
    <div
      className={`select__dropdown-content__item ${selected ? 'selected' : ''}`}
      onClick={() => onClick(value)}
    >
      <span>{label}</span>

      <IconCheck className="select__dropdown-content__item__icon" />
    </div>
  );
};

export const Select: FC<SelectProps> = ({
  items,
  selected,
  multiselect,
  placeholder,
  onChange,
}) => {
  const [isOpen, setIsOpen] = useState(false);

  const selectedItems = items.filter((item) =>
    multiselect ? selected?.includes(item.value) : selected === item.value,
  );

  const handleSelect = (value: string) => {
    if (multiselect) {
      onChange?.([...(selected || []), value]);
    } else {
      onChange?.(value);
    }
  };

  return (
    <Dropdown
      isOpen={isOpen}
      trigger={
        <Button
          rightIcon={
            <IconChevronDown
              className={`select__icon ${isOpen ? 'open' : ''}`}
            />
          }
        >
          {selectedItems.length
            ? selectedItems.map((item) => item.label).join(', ')
            : placeholder}
        </Button>
      }
      onShow={() => setIsOpen(true)}
      onHide={() => setIsOpen(false)}
    >
      {isOpen && (
        <div className="select__dropdown-content scrollbar">
          {items.map((item) => (
            <SelectItem
              {...item}
              key={item.value}
              selected={
                multiselect
                  ? selected?.includes(item.value) || false
                  : selected === item.value
              }
              onClick={() => handleSelect?.(item.value)}
            />
          ))}
        </div>
      )}
    </Dropdown>
  );
};
