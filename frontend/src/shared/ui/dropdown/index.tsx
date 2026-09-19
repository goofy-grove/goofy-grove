import {
  autoUpdate,
  flip,
  offset,
  shift,
  size,
  useFloating,
} from '@floating-ui/react';
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
  matchTriggerWidth = false,
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
    whileElementsMounted: autoUpdate,
    middleware: [
      offset(8),
      flip(),
      shift({ padding: 12 }),
      size({
        padding: 12,
        apply({ availableHeight, availableWidth, rects, elements }) {
          Object.assign(elements.floating.style, {
            maxHeight: `${Math.max(0, availableHeight)}px`,
            maxWidth: `${Math.max(0, availableWidth)}px`,
            minWidth: matchTriggerWidth
              ? `${Math.min(rects.reference.width, availableWidth)}px`
              : undefined,
          });
        },
      }),
    ],
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
    if (isDropdownOpen) {
      const target =
        contentRef.current?.querySelector<HTMLElement>(
          '[aria-selected="true"]',
        ) ??
        contentRef.current?.querySelector<HTMLElement>('button:not(:disabled)');

      target?.focus();
    }
  }, [isDropdownOpen]);

  useEffect(() => {
    if (!isDropdownOpen) return;

    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current?.contains(event.target as Node) ||
        contentRef.current?.contains(event.target as Node)
      ) {
        return;
      }

      handleHide();
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (
        event.key === 'Tab' &&
        contentRef.current?.contains(document.activeElement)
      ) {
        const buttons = Array.from(
          contentRef.current.querySelectorAll('button:not(:disabled)'),
        );
        const atEdge = event.shiftKey
          ? document.activeElement === buttons[0]
          : document.activeElement === buttons.at(-1);

        if (atEdge || contentRef.current.querySelector('[role="listbox"]')) {
          if (event.shiftKey) event.preventDefault();
          handleHide();
          dropdownRef.current?.querySelector('button')?.focus();
        }
      }

      if (event.key === 'Escape') {
        event.preventDefault();
        handleHide();
        dropdownRef.current?.querySelector('button')?.focus();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    document.addEventListener('click', handleClickOutside);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      document.removeEventListener('click', handleClickOutside);
    };
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
