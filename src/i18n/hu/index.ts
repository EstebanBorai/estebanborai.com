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
    HI: 'Jó napot kívánok {name:string} vagyok,',
    ABOUT: `Egy szoftverfejlesztő vagyok, aki a Rust, Svelte és a TypeScript iránt érdeklődik. Szenvedélyesen érdeklődöm a webfejlesztés és a rendszerprogramozás iránt.`,
    LATEST_NOTES: 'Legújabb jegyzetek',
  },
} satisfies BaseTranslation;

export default hu;
