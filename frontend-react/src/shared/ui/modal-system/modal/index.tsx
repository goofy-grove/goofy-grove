import { type FC } from 'react';

import { Card } from '../../card';
import { useModal } from '../hooks';
import { useDrag } from '../../hooks';

import type { ModalProps } from './types';

import './styles.scss';

export const Modal: FC<ModalProps> = ({ id, children }) => {
  const { closeModal, isModalOpen } = useModal();

  const handleModalClose = () => closeModal(id);

  const { componentRef, handleRef } = useDrag();

  return (
    isModalOpen(id) && (
      <Card
        className="modal"
        ref={componentRef}
        headerRef={handleRef}
        closable
        onClose={handleModalClose}
      >
        {children}
      </Card>
    )
  );
};
