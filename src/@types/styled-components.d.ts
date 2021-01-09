import 'styled-components';

declare module 'styled-components' {
  export interface DefaultTheme {
    fonts: {
      title: string;
      subtitle: string;
      headline: string;
      body: string;
      monospaced: string;
    };
    layoutBackground: string;
    textColor: string;
    primary: string;
  }
}
