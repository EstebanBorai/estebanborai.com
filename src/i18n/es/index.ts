import type { BaseTranslation } from '../i18n-types';

const es = {
  LAYOUT: {
    NAV: {
      HOME: 'Inicio',
      NOTES: 'Notas',
      BOOKMARKS: 'Marcadores',
    },
    FOOTER: {
      COPYRGHT: `Hecho con 🧉 y ❤️ por Esteban Borai © {start:number} - {end:number}`,
      SOURCE_CODE: 'Código Fuente',
    },
  },
  HOMEPAGE: {
    HI: `Hola soy {name:string} {surname:string},`,
    ABOUT: `Desarrollador de Software con experiencia en Web y Desarrollo de Sistemas`,
    SOCIAL_LINKS: 'Redes Sociales',
    LATEST_NOTES: 'Nota más reciente',
  },
} satisfies BaseTranslation;

export default es;
