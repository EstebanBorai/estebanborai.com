import type { BaseTranslation } from '../i18n-types';

const en = {
  LAYOUT: {
    NAV: {
      HOME: 'Home',
      NOTES: 'Notes',
    },
    FOOTER: {
      COPYRGHT: `Made with 🧉 and ❤️ by Esteban Borai © {start:number} - {end:number}`,
      SOURCE_CODE: 'Source Code',
    },
  },
  HOMEPAGE: {
    HI: `Hi I'm {name:string},`,
    ABOUT: `I'm a Software Developer focused on Rust, Svelte and TypeScript. I'm passionate about Web Development and Systems Programming.`,
    LATEST_NOTES: 'Latest Notes',
  },
} satisfies BaseTranslation;

export default en;
