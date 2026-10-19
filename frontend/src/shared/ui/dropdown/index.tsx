import {
  autoUpdate,
  flip,
  offset,
  shift,
  size,
  useFloating,
} from '@floating-ui/react';
import { useEffect, useRef, type FC } from 'react';
import { createPortal } from 'react-dom';

import type { DropdownProps } from './types';

import './styles.scss';

export const Dropdown: FC<DropdownProps> = ({
  children,
  isOpen,
  trigger,
  virtualReference,
  matchTriggerWidth = false,
  contentClass,
  onShow,
  onHide,
}) => {
  const dropdownRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);

  const { refs, floatingStyles } = useFloating({
    open: isOpen,
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

  const handleClick = (e: React.MouseEvent<HTMLDivElement>) => {
    e.stopPropagation();
    e.preventDefault();

    if (isOpen) {
      onHide?.();
    } else {
      onShow?.();
    }
  };

  useEffect(() => {
    refs.setPositionReference(virtualReference ?? dropdownRef.current);
  }, [refs, virtualReference]);

  const setReference = (node: HTMLDivElement | null) => {
    refs.setReference(node);
    dropdownRef.current = node;
  };

  const setFloating = (node: HTMLDivElement | null) => {
    refs.setFloating(node);
    contentRef.current = node;
  };

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current?.contains(event.target as Node) ||
        contentRef.current?.contains(event.target as Node)
      ) {
        return;
      }

      onHide?.();
    };

    document.addEventListener('click', handleClickOutside);

    return () => {
      document.removeEventListener('click', handleClickOutside);
    };
  });

  return (
    <div className="dropdown">
      {!virtualReference && (
        <div
          className="dropdown__trigger"
          onClick={handleClick}
          ref={setReference}
        >
          {trigger}
        </div>
      )}

      {isOpen &&
        createPortal(
          <div
            ref={setFloating}
            className={`dropdown__content ${contentClass || ''}`}
            style={floatingStyles}
          >
            {children}
          </div>,
          document.body,
        )}
    </div>
  );
};
