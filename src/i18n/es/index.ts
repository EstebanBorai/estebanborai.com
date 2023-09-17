import type { BaseTranslation } from '../i18n-types';

const es = {
  LAYOUT: {
    NAV: {
      HOME: 'Inicio',
      NOTES: 'Notas',
    },
    FOOTER: {
      COPYRGHT: `Hecho con 🧉 y ❤️ por Esteban Borai © {start:number} - {end:number}`,
      SOURCE_CODE: 'Código Fuente',
    },
  },
  HOMEPAGE: {
    HI: `Hola soy {name:string} {surname:string},`,
    ABOUT: `Soy un desarrollador de software con experiencia en <span class="hero-emphasis">Programación de Sistemas</span> y <span class="hero-emphasis">Desarrollo Web</span>.`,
    SOCIAL_LINKS: 'Redes Sociales',
    LATEST_NOTES: 'Nota más reciente',
  },
} satisfies BaseTranslation;

export default es;
