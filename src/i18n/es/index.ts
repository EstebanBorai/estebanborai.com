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
    HI: `Hola soy {name:string},`,
    ABOUT: `Soy Desarrollador de Software enfocado en Rust, Svelte y TypeScript. Me apasiona el desarrollo web y la programación de sistemas.`,
    LATEST_NOTES: 'Últimas notas',
  },
} satisfies BaseTranslation;

export default es;
