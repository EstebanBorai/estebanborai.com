import { createClient } from '@urql/svelte';

import type { Client } from '@urql/svelte';

export function makeUrqlClient(): Client {
	return createClient({
		url: import.meta.env.VITE_GRAPHQL_URL as string,
		fetchOptions: {
			mode: 'cors'
		}
	});
}

export const urqlClient = makeUrqlClient();
