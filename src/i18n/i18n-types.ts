// This file was auto-generated by 'typesafe-i18n'. Any manual changes will be overwritten.
/* eslint-disable */
import type {
  BaseTranslation as BaseTranslationType,
  LocalizedString,
  RequiredParams,
} from 'typesafe-i18n';

export type BaseTranslation = BaseTranslationType;
export type BaseLocale = 'en';

export type Locales = 'en' | 'es' | 'hu';

export type Translation = RootTranslation;

export type Translations = RootTranslation;

type RootTranslation = {
  LAYOUT: {
    NAV: {
      /**
       * H​o​m​e
       */
      HOME: string;
      /**
       * N​o​t​e​s
       */
      NOTES: string;
    };
    FOOTER: {
      /**
       * M​a​d​e​ ​w​i​t​h​ ​�​�​ ​a​n​d​ ​❤​️​ ​b​y​ ​E​s​t​e​b​a​n​ ​B​o​r​a​i​ ​©​ ​{​s​t​a​r​t​}​ ​-​ ​{​e​n​d​}
       * @param {number} end
       * @param {number} start
       */
      COPYRGHT: RequiredParams<'end' | 'start'>;
      /**
       * S​o​u​r​c​e​ ​C​o​d​e
       */
      SOURCE_CODE: string;
    };
  };
  HOMEPAGE: {
    /**
     * H​e​y​,​ ​I​'​m​ ​<​s​p​a​n​ ​c​l​a​s​s​=​"​f​o​n​t​-​s​e​m​i​b​o​l​d​"​>​{​n​a​m​e​}​ ​{​s​u​r​n​a​m​e​}​<​/​s​p​a​n​>​,
     * @param {string} name
     * @param {string} surname
     */
    HI: RequiredParams<'name' | 'surname'>;
    /**
     * I​'​m​ ​a​ ​S​o​f​t​w​a​r​e​ ​D​e​v​e​l​o​p​e​r​ ​w​i​t​h​ ​e​x​p​e​r​i​e​n​c​e​ ​o​n​ ​<​s​p​a​n​ ​c​l​a​s​s​=​"​h​e​r​o​-​e​m​p​h​a​s​i​s​"​>​S​y​s​t​e​m​s​ ​P​r​o​g​r​a​m​m​i​n​g​<​/​s​p​a​n​>​ ​a​n​d​ ​<​s​p​a​n​ ​c​l​a​s​s​=​"​h​e​r​o​-​e​m​p​h​a​s​i​s​"​>​W​e​b​ ​D​e​v​e​l​o​p​m​e​n​t​<​/​s​p​a​n​>​.
     */
    ABOUT: string;
    /**
     * S​o​c​i​a​l​ ​N​e​t​w​o​r​k​s
     */
    SOCIAL_LINKS: string;
    /**
     * L​a​t​e​s​t​ ​N​o​t​e
     */
    LATEST_NOTES: string;
  };
};

export type TranslationFunctions = {
  LAYOUT: {
    NAV: {
      /**
       * Home
       */
      HOME: () => LocalizedString;
      /**
       * Notes
       */
      NOTES: () => LocalizedString;
    };
    FOOTER: {
      /**
       * Made with 🧉 and ❤️ by Esteban Borai © {start} - {end}
       */
      COPYRGHT: (arg: { end: number; start: number }) => LocalizedString;
      /**
       * Source Code
       */
      SOURCE_CODE: () => LocalizedString;
    };
  };
  HOMEPAGE: {
    /**
     * Hey, I'm <span class="font-semibold">{name} {surname}</span>,
     */
    HI: (arg: { name: string; surname: string }) => LocalizedString;
    /**
     * I'm a Software Developer with experience on <span class="hero-emphasis">Systems Programming</span> and <span class="hero-emphasis">Web Development</span>.
     */
    ABOUT: () => LocalizedString;
    /**
     * Social Networks
     */
    SOCIAL_LINKS: () => LocalizedString;
    /**
     * Latest Note
     */
    LATEST_NOTES: () => LocalizedString;
  };
};

export type Formatters = {};
