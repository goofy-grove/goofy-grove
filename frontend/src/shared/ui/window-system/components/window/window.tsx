import { useState, type FC } from 'react';
import { createPortal } from 'react-dom';
import { Rnd, type RndResizeCallback } from 'react-rnd';
import { IconMaximize, IconMinimize } from '@tabler/icons-react';
import { useMediaQuery } from 'react-responsive';

import { Card } from '../../../card';
import { Button } from '../../../button';
import { useCurrentWindow } from '../../hooks';

import type { WindowProps } from './types';

import './styles.scss';

export const Window: FC<WindowProps> = ({
  children,
  withoutHeader,
  title,
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

  const [width, setWidth] = useState(600);
  const [height, setHeight] = useState(400);

  // FIXME: This is a temporary solution to disable resizing and dragging on mobile devices
  // need to use hook without raw value
  const isMobile = useMediaQuery({ query: '(max-width: 768px)' });

  const handleResize: RndResizeCallback = (_e, _dir, ref) => {
    setWidth(ref.offsetWidth);
    setHeight(ref.offsetHeight);
  };

  const defaultParameters = {
    x: 100,
    y: 100,
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
      minHeight={200}
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
