export interface AppModalProps {
  initialX?: number;
  initialY?: number;
  showClose?: boolean;
  showHeader?: boolean;
  disableMove?: boolean;
  isOpen: boolean;
}

export interface AppModalEmits {
  (e: 'close'): void;
}

export interface AppModalSlots {
  title: () => void;
  default: () => void;
}
