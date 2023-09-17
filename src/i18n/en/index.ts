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
    ABOUT: `I'm a Software Developer with experience on <span class="hero-emphasis">Systems Programming</span> and <span class="hero-emphasis">Web Development</span>.`,
    SOCIAL_LINKS: 'Social Networks',
    LATEST_NOTES: 'Latest Note',
  },
} satisfies BaseTranslation;

export default en;
