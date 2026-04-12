import {
  IconAlertTriangleFilled,
  IconCircleCheckFilled,
  IconCircleXFilled,
  IconInfoCircleFilled,
  IconX,
  type IconProps,
} from '@tabler/icons-react';
import { useState, type FC, type ReactNode } from 'react';

import { Button } from '@shared/ui/button';
import { Text } from '@shared/ui/text';

import type { AlertProps } from './types';

import './styles.scss';

export const Alert: FC<AlertProps> = ({ message, type = 'info', closable }) => {
  const [isOpen, setIsOpen] = useState(true);
  const typeIconMap: Record<typeof type, (props: IconProps) => ReactNode> = {
    success: IconCircleCheckFilled,
    error: IconCircleXFilled,
    info: IconInfoCircleFilled,
    warning: IconAlertTriangleFilled,
  };

  const Icon = typeIconMap[type];

  return (
    isOpen && (
      <div className={`alert ${type}`}>
        <Icon className="alert__icon" size={32} />

        <Text tag="span" className="alert__message">
          {message}
        </Text>

        {closable && (
          <Button
            className="alert__close"
            variant="ghost"
            rightIcon={<IconX />}
            onClick={() => setIsOpen(false)}
          />
        )}
      </div>
    )
  );
};
