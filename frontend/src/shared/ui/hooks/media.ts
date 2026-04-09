import { useMediaQuery } from 'react-responsive';

export const useBreakpoints = () => {
  const computedStyle = getComputedStyle(document.documentElement);

  const mobileXs = computedStyle.getPropertyValue('--mobile-xs').trim();
  const mobileSm = computedStyle.getPropertyValue('--mobile-sm').trim();
  const tabletSm = computedStyle.getPropertyValue('--tablet-sm').trim();
  const tabletLg = computedStyle.getPropertyValue('--tablet-lg').trim();
  const desktopSm = computedStyle.getPropertyValue('--desktop-sm').trim();

  return {
    isMobileXs: useMediaQuery({ query: `screen and (max-width: ${mobileXs})` }),
    isMobileSm: useMediaQuery({ query: `screen and (max-width: ${mobileSm})` }),
    isTabletSm: useMediaQuery({ query: `screen and (max-width: ${tabletSm})` }),
    isTabletLg: useMediaQuery({ query: `screen and (max-width: ${tabletLg})` }),
    isDesktopSm: useMediaQuery({
      query: `screen and (max-width: ${desktopSm})`,
    }),
  };
};
