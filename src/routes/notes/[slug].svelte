<script context="module" lang="ts">
	import { urqlClient } from '$lib/utils/urql';

	const notesQuery = `
query($slug: String!) {
  notes(slug: $slug) {
    edges {
      node {
        id
        title
        date
        description
        slug
        categories
        content
        previewImageUrl
      }
    }
  }
}
`;

	export async function load({ fetch, params }: LoadInput): Promise<LoadOutput> {
		const result = await urqlClient
			.query(
				notesQuery,
				{
					slug: params.slug
				},
				{
					fetch
				}
			)
			.toPromise();

		if (result.error) {
			return {
				props: {
					note: null,
					error: result.error
				}
			};
		}

		if (result.data.notes?.edges?.length) {
			return {
				props: {
					note: result.data.notes.edges[0].node,
					error: result.error
				}
			};
		}
	}
</script>

<script lang="ts">
	import { Remarkable } from 'remarkable';

	import type { LoadOutput, LoadInput } from '@sveltejs/kit';

	export let error = null;
	export let note = null;

	const markdown = new Remarkable();
	const content = markdown.render(note.content);
</script>

<svelte:head>
	{#if note}
		<title>{note.title} | Esteban Borai</title>
	{/if}
</svelte:head>

{#if error}
	{JSON.stringify(error)}
{:else}
	<style>
		.note-container {
			@apply px-4;
		}

		.note-container img {
			@apply p-4;
		}

		.note-container h2 {
			@apply py-4 text-xl;
		}

		.note-container ul {
			@apply pl-4 my-4;

			list-style: inherit;
		}

		.note-container code {
			@apply p-1 rounded border;

			background: gray;
			box-sizing: border-box;
		}

		.note-container pre code {
			@apply my-4;

			background-color: grey;
			display: inline-block;
			overflow-x: scroll;
			width: 100%;
		}
	</style>

	<div class="note-container">
		<header class="post-header">
			<h1 class="text-3xl py-4">{note.title}</h1>
		</header>
		{@html content}
	</div>
{/if}
