const MONTHS = [
  'January',
  'February',
  'March',
  'April',
  'May',
  'June',
  'July',
  'August',
  'September',
  'October',
  'November',
  'December',
];

export function humanDate(date: Date): string {
  const month = MONTHS[date.getMonth()];
  const year = date.getFullYear();

  let day: string | number = date.getDate() + 1;

  // Two digits padding
  if (day < 10) {
    day = '0' + day;
  }

  switch (day) {
    case 1:
      day = '1st';
      break;

    case 2:
      day = '2nd';
      break;

    case 3:
      day = '3rd';
      break;

    default:
      day = day + 'th';
      break;
  }

  return month + ' ' + day + ', ' + year;
}
