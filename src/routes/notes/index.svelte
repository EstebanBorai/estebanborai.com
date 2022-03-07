<script lang="ts" context="module">
  export async function load({ fetch }) {
    const url = '/api/notes.json';
    const res = await fetch(url);
    if (res.ok) {
      const { notes } = await res.json();
      return {
        props: {
          notes,
        },
      };
    }

    return {
      status: res.status,
      error: new Error(`Could not load ${url}`),
    };
  }
</script>

<script>
  import Entry from '$lib/modules/notes/components/Entry.svelte';

  export let notes;
</script>

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
