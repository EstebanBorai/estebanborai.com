import type { Locales } from '$i18n/i18n-types';

const MONTHS = [
  {
    en: 'January',
    es: 'Enero',
    hu: 'Január',
  },
  {
    en: 'February',
    es: 'Febrero',
    hu: 'Február',
  },
  {
    en: 'March',
    es: 'Marzo',
    hu: 'Március',
  },
  {
    en: 'April',
    es: 'Abril',
    hu: 'Április',
  },
  {
    en: 'May',
    es: 'Mayo',
    hu: 'Május',
  },
  {
    en: 'June',
    es: 'Junio',
    hu: 'Június',
  },
  {
    en: 'July',
    es: 'Julio',
    hu: 'Július',
  },
  {
    en: 'August',
    es: 'Agosto',
    hu: 'Augusztus',
  },
  {
    en: 'September',
    es: 'Septiembre',
    hu: 'Szeptember',
  },
  {
    en: 'October',
    es: 'Octubre',
    hu: 'Október',
  },
  {
    en: 'November',
    es: 'Noviembre',
    hu: 'November',
  },
  {
    en: 'December',
    es: 'Diciembre',
    hu: 'December',
  },
];

export function humanDate(locale: Locales, date: Date): string {
  const month = MONTHS[date.getMonth()];
  const year = date.getFullYear();

  let day: string | number = date.getDate() + 1;

  // Two digits padding
  if (day < 10) {
    day = '0' + day;
  }

  switch (day) {
    case 1:
      if (locale === 'hu') {
        day = '1';
        break;
      }

      if (locale === 'es') {
        day = '1ro';
        break;
      }

      day = '1st';
      break;

    case 2:
      if (locale === 'hu') {
        day = '2';
        break;
      }

      if (locale === 'es') {
        day = '2do';
        break;
      }

      day = '2nd';
      break;

    case 3:
      if (locale === 'hu') {
        day = '3';
        break;
      }

      if (locale === 'es') {
        day = '3ro';
        break;
      }

      day = '3rd';
      break;

    default:
      if (locale != 'en') {
        break;
      }

      day = day + 'th';
      break;
  }

  const localeMonth = month[locale];

  if (locale === 'hu') {
    return `${year}. ${localeMonth} ${day}`;
  }

  if (locale === 'es') {
    return `${day} de ${localeMonth} de ${year}`;
  }

  return `${localeMonth} ${day}, ${year}`;
}
