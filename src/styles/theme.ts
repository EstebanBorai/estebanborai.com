import type { DefaultTheme } from 'styled-components';

type ApplicationTheme = {
  light: DefaultTheme;
  dark: DefaultTheme;
};

type CSSBreakpoints = {
  xs: number;
  sm: number;
  md: number;
  lg: number;
  xl: number;
  xxl: number;
};

type CSSMediaQuery = {
  xs: string;
  sm: string;
  md: string;
  lg: string;
  xl: string;
  xxl: string;
};

export const cssBreakpoint: CSSBreakpoints = {
  xs: 320,
  sm: 375,
  md: 768,
  lg: 1024,
  xl: 1280,
  xxl: 1536,
};

export const media: CSSMediaQuery = {
  xs: `@media screen and (max-width: ${cssBreakpoint.xs}px)`,
  sm: `@media screen and (max-width: ${cssBreakpoint.sm}px)`,
  md: `@media screen and (max-width: ${cssBreakpoint.md}px)`,
  lg: `@media screen and (max-width: ${cssBreakpoint.lg}px)`,
  xl: `@media screen and (max-width: ${cssBreakpoint.xl}px)`,
  xxl: `@media screen and (max-width: ${cssBreakpoint.xxl}px)`,
};

const fonts = {
  title: "'Calistoga', display",
  subtitle: "'Alata', sans-serif",
  headline: "'Alata', sans-serif",
  body: "'Source Serif Pro', serif",
  monospaced: "'Inconsolata', monospace",
};

const applicationTheme: ApplicationTheme = {
  light: {
    fonts,
    layoutBackground: '#ffffff',
    textColor: '#111111',
    primary: 'red',
  },
  dark: {
    fonts,
    layoutBackground: '#212121',
    textColor: '#fefefe',
    primary: 'blue',
  },
};

export default applicationTheme;
