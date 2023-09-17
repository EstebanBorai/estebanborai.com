import type { BaseTranslation } from '../i18n-types';

const hu = {
  LAYOUT: {
    NAV: {
      HOME: 'Főoldal',
      NOTES: 'Jegyzet',
    },
    FOOTER: {
      COPYRGHT: `Készült 🧉 és ❤️ írta Esteban Borai © {start:number} - {end:number}`,
      SOURCE_CODE: 'Forráskód',
    },
  },
  HOMEPAGE: {
    HI: 'Jó napot kívánok {surname:string} {name:string} vagyok,',
    ABOUT: `Egy szoftverfejlesztő vagyok aki tapasztalattal rendelkezik <span class="hero-emphasis">Rendszerprogramozás</span> és <span class="hero-emphasis">Webfejlesztés</span> területén.`,
    SOCIAL_LINKS: 'Közösségi hálók',
    LATEST_NOTES: 'Legújabb jegyzet',
  },
} satisfies BaseTranslation;

export default hu;
