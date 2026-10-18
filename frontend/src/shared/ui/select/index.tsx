import { IconCheck, IconChevronDown } from '@tabler/icons-react';
import { useId, useState, type FC } from 'react';

import { Dropdown } from '@shared/ui/dropdown';

import type { SelectProps } from './types';

import './styles.scss';

export const Select: FC<SelectProps> = ({
  items,
  className,
  renderOption,
  renderValue,
  selected,
  multiselect,
  placeholder,
  onChange,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const listId = useId();
  const selectedItems = items.filter((item) =>
    multiselect ? selected?.includes(item.value) : selected === item.value,
  );

  const handleSelect = (value: string) => {
    if (multiselect) {
      const values = selected ?? [];

      onChange?.(
        values.includes(value)
          ? values.filter((item) => item !== value)
          : [...values, value],
      );
    } else {
      onChange?.(value);
      setIsOpen(false);
    }
  };

  return (
    <div className={`select ${className ?? ''}`}>
      <Dropdown
        isOpen={isOpen}
        matchTriggerWidth
        onShow={() => setIsOpen(true)}
        onHide={() => setIsOpen(false)}
        trigger={
          <button
            type="button"
            className={`select__trigger ${isOpen ? 'select__trigger--open' : ''}`}
            aria-label={placeholder}
            aria-haspopup="listbox"
            aria-expanded={isOpen}
            aria-controls={isOpen ? listId : undefined}
          >
            <span
              className={
                selectedItems.length ? 'select__value' : 'select__placeholder'
              }
            >
              {selectedItems.length
                ? (renderValue?.(selectedItems) ??
                  selectedItems.map((item) => item.label).join(', '))
                : placeholder}
            </span>

            <IconChevronDown
              size={18}
              className="select__chevron"
              aria-hidden="true"
            />
          </button>
        }
      >
        <div
          id={listId}
          role="listbox"
          aria-label={placeholder}
          aria-multiselectable={multiselect || undefined}
          className="select__options"
        >
          {items.map((item) => {
            const isSelected = selectedItems.some(
              (value) => value.value === item.value,
            );

            return (
              <button
                key={item.value}
                type="button"
                role="option"
                aria-selected={isSelected}
                className="select__option"
                onClick={() => handleSelect(item.value)}
              >
                <span>{renderOption?.(item) ?? item.label}</span>

                <IconCheck
                  size={17}
                  className="select__check"
                  aria-hidden="true"
                />
              </button>
            );
          })}
        </div>
      </Dropdown>
    </div>
  );
};
