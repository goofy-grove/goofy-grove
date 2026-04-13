import { IconMaximize, IconMinimize } from '@tabler/icons-react';
import { useState, type FC } from 'react';
import { createPortal } from 'react-dom';
import { useMediaQuery } from 'react-responsive';
import { Rnd, type RndResizeCallback } from 'react-rnd';

import { Button } from '@shared/ui/button';
import { Card } from '@shared/ui/card';
import { useCurrentWindow } from '@shared/ui/window-system/hooks';

import type { WindowProps } from './types';

import './styles.scss';

export const Window: FC<WindowProps> = ({
  children,
  withoutHeader,
  title,
  defaultWidth,
  defaultHeight,
  defaultX,
  defaultY,
  bounds = document.body,
}) => {
  const {
    instanceId: id,
    zIndex,
    closeWindow,
    maximizeWindow,
    minimizeWindow,
    updateLastInteraction: handleWindowInteract,
    isMaximized,
  } = useCurrentWindow();

  const handleWindowClose = () => {
    closeWindow();
  };

  const [width, setWidth] = useState(defaultWidth || 600);
  const [height, setHeight] = useState(defaultHeight || 400);

  // FIXME: This is a temporary solution to disable resizing and dragging on mobile devices
  // need to use hook without raw value
  const isMobile = useMediaQuery({ query: '(max-width: 768px)' });

  const handleResize: RndResizeCallback = (_e, _dir, ref) => {
    setWidth(ref.offsetWidth);
    setHeight(ref.offsetHeight);
  };

  const defaultParameters = {
    x: defaultX || 100,
    y: defaultY || 100,
    width,
    height,
  };

  return createPortal(
    <Rnd
      className={`window__wrapper ${isMaximized || isMobile ? 'expanded' : ''}`}
      enableResizing={!isMobile && !isMaximized}
      disableDragging={isMaximized || isMobile}
      bounds={bounds}
      default={defaultParameters}
      onResize={handleResize}
      onResizeStart={() => handleWindowInteract()}
      minWidth={300}
      minHeight={65}
      dragHandleClassName={
        withoutHeader ? `__card-${id}` : `__card-header-${id}`
      }
      style={{ zIndex }}
    >
      <Card
        id={id}
        className={`window ${isMaximized || isMobile ? 'expanded' : ''}`}
        title={title}
        withoutHeader={withoutHeader}
        closable
        onClose={handleWindowClose}
        onPointerDown={handleWindowInteract}
        actions={
          <Button
            variant="ghost"
            disabled={isMobile}
            onClick={() => (isMaximized ? minimizeWindow() : maximizeWindow())}
            rightIcon={
              isMaximized || isMobile ? <IconMinimize /> : <IconMaximize />
            }
          />
        }
      >
        {children}
      </Card>
    </Rnd>,
    document.body,
  );
};
