import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export async function load({ params }) {
  throw redirect(301, `/${params.lang}/notes`);
}
