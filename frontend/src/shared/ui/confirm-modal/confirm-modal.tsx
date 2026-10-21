import { type FC } from 'react';

import { Button } from '@shared/ui/button';
import { Modal } from '@shared/ui/modal';
import { Text } from '@shared/ui/text';

import './styles.scss';
import type { ConfirmModalProps } from './types';

export const ConfirmModal: FC<ConfirmModalProps> = ({
  isOpen,
  title,
  message,
  confirmLabel,
  cancelLabel,
  confirmColor = 'error',
  isPending = false,
  onConfirm,
  onCancel,
}) => (
  <Modal
    isOpen={isOpen}
    title={
      <Text tag="h3" className="confirm-modal__title">
        {title ?? message}
      </Text>
    }
    onClose={onCancel}
  >
    <div className="confirm-modal">
      {title ? (
        <Text className="confirm-modal__message" variant="secondary">
          {message}
        </Text>
      ) : null}

      <div className="confirm-modal__actions">
        <Button variant="ghost" disabled={isPending} onClick={onCancel}>
          {cancelLabel}
        </Button>

        <Button color={confirmColor} disabled={isPending} onClick={onConfirm}>
          {confirmLabel}
        </Button>
      </div>
    </div>
  </Modal>
);
