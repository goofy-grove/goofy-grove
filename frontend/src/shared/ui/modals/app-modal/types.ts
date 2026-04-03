import type { ClassValue } from 'vue';

export interface AppModalProps {
  class?: ClassValue;
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
  title?: () => void;
  default: () => void;
}
