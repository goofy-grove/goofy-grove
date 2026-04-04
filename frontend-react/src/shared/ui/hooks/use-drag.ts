import { useEffect, useRef } from 'react';

export const useDrag = () => {
  const handleRef = useRef<HTMLDivElement>(null);
  const componentRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handle = handleRef.current;
    const component = componentRef.current;

    if (!handle || !component) {
      return;
    }

    component.style.transform = `translate(${0}px, ${0}px)`;

    const handlePointerDown = (event: PointerEvent) => {
      event.preventDefault();
      event.stopPropagation();

      const shiftX = event.clientX - handle.getBoundingClientRect().left;
      const shiftY = event.clientY - handle.getBoundingClientRect().top;

      const onPointerMove = (event: PointerEvent) => {
        event.preventDefault();
        event.stopPropagation();

        const left = event.clientX - shiftX;
        const top = event.clientY - shiftY;

        component.style.transform = `translate(${left}px, ${top}px)`;
      };

      const onPointerUp = () => {
        document.removeEventListener('pointermove', onPointerMove);
        document.removeEventListener('pointerup', onPointerUp);
      };

      document.addEventListener('pointermove', onPointerMove);
      document.addEventListener('pointerup', onPointerUp);
    };

    handle.addEventListener('pointerdown', handlePointerDown);

    return () => {
      handle.removeEventListener('pointerdown', handlePointerDown);
    };
  });

  return {
    handleRef,
    componentRef,
  };
};
