import { useState, type FC } from 'react';
import { createPortal } from 'react-dom';
import { Rnd, type RndResizeCallback } from 'react-rnd';
import { IconMaximize, IconMinimize } from '@tabler/icons-react';
import { useMediaQuery } from 'react-responsive';

import { Card } from '../../../card';
import { Button } from '../../../button';

import type { WindowProps } from './types';

import './styles.scss';

export const Window: FC<WindowProps> = ({
  id,
  children,
  withoutHeader,
  title,
  onClose,
  bounds = document.body,
}) => {
  const handleWindowClose = () => {
    onClose?.();
  };

  const [width, setWidth] = useState(600);
  const [height, setHeight] = useState(400);

  const [isExpanded, setIsExpanded] = useState(false);

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

  const resizeParamters =
    isMobile || isExpanded
      ? { top: false, bottom: false, left: false, right: false }
      : { top: true, bottom: true, left: true, right: true };

  return createPortal(
    <Rnd
      className={`window__wrapper ${isExpanded || isMobile ? 'expanded' : ''}`}
      enableResizing={resizeParamters}
      disableDragging={isExpanded || isMobile}
      bounds={bounds}
      default={defaultParameters}
      onResize={handleResize}
      minWidth={300}
      minHeight={200}
      dragHandleClassName={
        withoutHeader ? `__card-${id}` : `__card-header-${id}`
      }
    >
      <Card
        id={id}
        className={`window ${isExpanded || isMobile ? 'expanded' : ''}`}
        style={{
          width,
          height,
        }}
        title={title}
        withoutHeader={withoutHeader}
        closable
        onClose={handleWindowClose}
        actions={
          <Button
            variant="ghost"
            disabled={isMobile}
            onClick={() => setIsExpanded(!isExpanded)}
            rightIcon={
              isExpanded || isMobile ? <IconMinimize /> : <IconMaximize />
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
