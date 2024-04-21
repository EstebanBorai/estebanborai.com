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
    ABOUT: `Szoftvermérnök webes és rendszertapasztalattal. A kedvenc nyelv a ', <span class="hero-emphasis-orange">Rust</span> Rust, de én is <span class="hero-emphasis-blue">TypeScript</span> és <span class="hero-emphasis-red">Svelte</span>.`,
    SOCIAL_LINKS: 'Közösségi hálók',
    LATEST_NOTES: 'Legújabb jegyzet',
  },
} satisfies BaseTranslation;

export default hu;
