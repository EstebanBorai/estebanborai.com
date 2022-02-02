<script lang="ts">
	import { onMount } from 'svelte';

	import Entry from '$lib/modules/notes/components/Entry.svelte';
	import { urqlClient } from '$lib/utils/urql';
	import { captureException } from '@sentry/browser';

	let error = null;
	let notes = [];
	let isLoading = false;

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
			}`;

	onMount(async () => {
		try {
			isLoading = true;
			const result = await urqlClient.query(notesQuery, null).toPromise();

			if (result.data?.notes?.edges?.length) {
				notes = result.data.notes.edges.map(({ node }) => node);
				error = result.error;
				return;
			}

			notes = [];
			error = result.error;
		} catch (err) {
			captureException(err);
			notes = [];
			error = 'An unhandled error ocurred!';
		} finally {
			isLoading = false;
		}
	});
</script>

{#if isLoading}
	<h1>Loading</h1>
{:else if error}
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
