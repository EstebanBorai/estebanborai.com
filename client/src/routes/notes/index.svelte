<script context="module" lang="ts">
	import { urqlClient } from '$lib/utils/urql';

	const notesQuery = `
query {
  notes {
    edges {
      node {
        id
        title
        date
        description
        slug
        categories
      }
    }
  }
}
`;

	export async function load({ fetch }: LoadInput): Promise<LoadOutput> {
		const result = await urqlClient
			.query(notesQuery, null, {
				fetch
			})
			.toPromise();

		if (result.data.notes?.edges?.length) {
			return {
				props: {
					notes: result.data.notes.edges.map(({ node }) => node),
					error: result.error
				}
			};
		}

		return {
			props: {
				notes: [],
				error: result.error
			}
		};
	}
</script>

<script lang="ts">
	import Entry from '$lib/modules/notes/components/Entry.svelte';

	import type { LoadOutput, LoadInput } from '@sveltejs/kit';

	export let error = null;
	export let notes = [];
</script>

{#if error}
	{error}
{:else}
	<ul>
		{#each notes as note}
			<Entry
				title={note.title}
				description={note.description}
				publishDate={new Date(note.date)}
				tags={note.categories}
				slug={note.slug}
			/>
		{/each}
	</ul>
{/if}
