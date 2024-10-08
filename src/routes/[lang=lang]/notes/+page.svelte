<script lang="ts">
  import { onMount } from 'svelte';

  import { locale } from '$i18n/i18n-svelte';
  import Entry from './components/Entry.svelte';
  import Button from '$lib/components/Button.svelte';

  export let data: { notesIndex: Domain.BlogNote[] };

  let currentPage = 1;
  let hasNextPage = true;
  let title = 'Esteban Borai | Notes';
  let description =
    'Notes taken while reading about computer science and software development.';
  let avatarUrl = 'https://avatars.githubusercontent.com/u/34756077?v=4';
  let notesIndex = [];

  const PAGE_SIZE = 9;

  function nextPage(page: number): any {
    const pageSize = PAGE_SIZE;

    return {
      notes: data.notesIndex.slice((page - 1) * pageSize, page * pageSize),
      hasNext: data.notesIndex.length > page * pageSize,
    };
  }

  function handleShowMore(): void {
    const { notes, hasNext } = nextPage(currentPage + 1);

    currentPage += 1;
    hasNextPage = hasNext;
    notesIndex = [...notesIndex, notes];
  }

  onMount(() => {
    const { notes, hasNext } = nextPage(currentPage);

    hasNextPage = hasNext;
    notesIndex = [notes];
  });

  $: {
    switch ($locale) {
      case 'en':
        title = 'Esteban Borai | Notes';
        description =
          'Notes taken while reading about computer science and software development.';
        break;
      case 'es':
        title = 'Esteban Borai | Notas';
        description =
          'Notas tomadas mientras leo sobre ciencias de la computación y desarrollo de software.';
        break;
      case 'hu':
        title = 'Borai Esteban | Jegyzetek';
        description =
          'Jegyzetek, amelyeket olvasás közben vettem fel a számítástechnikáról és a szoftverfejlesztésről.';
        break;
    }
  }
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="description" content={description} />
  <!-- Schema.org markup for Google+ -->
  <meta itemprop="name" content={title} />
  <meta itemprop="description" content={description} />
  <meta itemprop="image" content={avatarUrl} />
  <!-- Open Graph data -->
  <meta property="og:title" content={title} />
  <meta property="og:type" content="article" />
  <meta property="og:url" content="https://estebanborai.com/" />
  <meta property="og:image" content={avatarUrl} />
  <meta property="og:description" content={description} />
  <meta property="og:site_name" content="Esteban Borai" />
  <!-- Twitter Card data -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:site" content="@EstebanBorai" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:creator" content="@EstebanBorai" />
  <meta name="twitter:image:src" content={avatarUrl} />
</svelte:head>

<section class="grid gap-y-8 gap-x-6 grid-cols-4 md:grid-cols-12">
  {#each notesIndex as page}
    {#each page as { meta, slug }, idx}
      <Entry
        title={meta.title}
        description={meta.description}
        publishDate={new Date(meta.date)}
        tags={meta.categories}
        {slug}
      />
    {/each}
  {/each}
</section>
{#if hasNextPage}
  <div class="flex justify-center items-center py-6 w-full">
    <Button
      title="Show more"
      disabled={!hasNextPage}
      on:click={handleShowMore}
    />
  </div>
{/if}
