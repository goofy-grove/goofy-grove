import { type FC } from 'react';
import { createPortal } from 'react-dom';

import { useHotkey } from '@shared/hotkeys';

import type { ModalProps } from './types';

import './styles.scss';

export const Modal: FC<ModalProps> = ({
  isOpen,
  title,
  children,
  className,
  onClose,
}) => {
  useHotkey('modal.close', onClose, { enabled: isOpen });

  if (!isOpen) {
    return null;
  }

  return createPortal(
    <div className="modal">
      <div className="modal__backdrop" onClick={onClose} />

      <div
        className={`modal__dialog ${className ?? ''}`}
        role="dialog"
        aria-modal="true"
      >
        {title}

        <div className="modal__body">{children}</div>
      </div>
    </div>,
    document.body,
  );
};
