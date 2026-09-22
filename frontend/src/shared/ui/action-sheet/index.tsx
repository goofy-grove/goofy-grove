import { createPortal } from 'react-dom';
import { useTranslation } from 'react-i18next';

import { Button } from '@shared/ui/button';

import type { ActionSheetProps } from './types';

import './styles.scss';

export const ActionSheet = ({
  isOpened,
  items,
  closeTitle,
  showClose,
  onClose,
}: ActionSheetProps) => {
  const { t } = useTranslation();

  const handleClose = (event: React.MouseEvent<HTMLDivElement>) => {
    if (event.target === event.currentTarget) {
      onClose?.();
    }
  };

  return createPortal(
    <div
      className={`action-sheet__wrapper ${isOpened ? 'action-sheet__wrapper--opened' : ''}`}
      onClick={handleClose}
    >
      <div className={`action-sheet ${isOpened ? 'action-sheet--opened' : ''}`}>
        <div className="action-sheet__content">
          {items.map((item, index) => (
            <Button
              key={index}
              className="action-sheet__item"
              variant="ghost"
              leftIcon={item.Icon}
              color={item.color}
              onClick={item.onClick}
            >
              {item.title}
            </Button>
          ))}
        </div>

        {showClose && (
          <Button className="action-sheet__close" onClick={onClose}>
            {closeTitle ?? t('common.close')}
          </Button>
        )}
      </div>
    </div>,
    document.body,
  );
};
