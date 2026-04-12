import { offset, useFloating } from '@floating-ui/react';
import { useEffect, useRef, useState, type FC } from 'react';
import { createPortal } from 'react-dom';

import type { DropdownProps } from './types';

import './styles.scss';

export const Dropdown: FC<DropdownProps> = ({
  children,
  isOpen,
  trigger,
  onShow,
  onHide,
}) => {
  const dropdownRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);

  const [isDropdownOpen, setIsDropdownOpen] = useState(isOpen);

  useEffect(() => {
    setIsDropdownOpen(isOpen);
  }, [isOpen]);

  const { refs, floatingStyles } = useFloating({
    open: isDropdownOpen,
    placement: 'bottom-start',
    middleware: [offset(4)],
  });

  const handleShow = () => {
    setIsDropdownOpen(true);
    onShow?.();
  };

  const handleHide = () => {
    setIsDropdownOpen(false);
    onHide?.();
  };

  const handleClick = (e: React.MouseEvent<HTMLDivElement>) => {
    e.stopPropagation();
    e.preventDefault();

    if (isDropdownOpen) {
      handleHide();
    } else {
      handleShow();
    }
  };

  const setReference = (node: HTMLDivElement) => {
    refs.setReference(node);
    dropdownRef.current = node;
  };

  const setFloating = (node: HTMLDivElement) => {
    refs.setFloating(node);
    contentRef.current = node;
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current?.contains(event.target as Node) ||
        contentRef.current?.contains(event.target as Node)
      ) {
        return;
      }

      handleHide();
    };

    document.addEventListener('click', handleClickOutside);

    return () => document.removeEventListener('click', handleClickOutside);
  });

  return (
    <div className="dropdown">
      <div
        className="dropdown__trigger"
        onClick={handleClick}
        ref={setReference}
      >
        {trigger}
      </div>

      {isDropdownOpen &&
        createPortal(
          <div
            ref={setFloating}
            className="dropdown__content"
            style={floatingStyles}
          >
            {children}
          </div>,
          document.body,
        )}
    </div>
  );
};
