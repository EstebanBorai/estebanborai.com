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
    ABOUT: `Software Developer with Web & Systems Experience. Favourite language is <span class="hero-emphasis-orange">Rust</span> but I also do <span class="hero-emphasis-blue">TypeScript</span> and <span class="hero-emphasis-red">Svelte</span>.`,
    SOCIAL_LINKS: 'Social Networks',
    LATEST_NOTES: 'Latest Note',
  },
} satisfies BaseTranslation;

export default en;
