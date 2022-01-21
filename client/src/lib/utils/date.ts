const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];

export function humanDate(date: Date): string {
	const month = MONTHS[date.getMonth()];
	const year = date.getFullYear();

	let day: string | number = date.getDate();

	// Two digits padding
	if (day < 10) {
		day = '0' + day;
	}

	return month + ' ' + day + ', ' + year;
}
