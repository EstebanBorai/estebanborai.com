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
    HI: `Hey, I'm <span class="font-semibold">{name:string} {surname:string}</span>,`,
    ABOUT: `Software Developer with Web & Systems Experience`,
    SOCIAL_LINKS: 'Social Networks',
    LATEST_NOTES: 'Latest Note',
  },
} satisfies BaseTranslation;

export default en;
