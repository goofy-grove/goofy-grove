export type WindowInstance = {
  instanceId: string;
  type: string;
  isMaximized: boolean;
  lastInteraction: number;
  props?: Record<string, unknown>;
};
