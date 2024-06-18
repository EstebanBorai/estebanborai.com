<script lang="ts">
  import { onMount } from 'svelte';

  import { locale } from '$i18n/i18n-svelte';
  import Bookmark from './components/Bookmark.svelte';
  import Button from '$lib/components/Button.svelte';

  export let data: { bookmarks: object[] };

  let currentPage = 1;
  let hasNextPage = true;
  let title = 'Esteban Borai | Bookmarks';
  let description =
    'Bookmarks from relevant articles, blog posts, and documentation.';
  let avatarUrl = 'https://avatars.githubusercontent.com/u/34756077?v=4';
  let bookmarks = [];

  function nextPage(page: number): any {
    const pageSize = 6;

    return {
      bookmarks: data.bookmarks.slice((page - 1) * pageSize, page * pageSize),
      hasNext: data.bookmarks.length > page * pageSize,
    };
  }

  function handleShowMore(): void {
    const { bookmarks: bookmarksList, hasNext } = nextPage(currentPage + 1);

    currentPage += 1;
    hasNextPage = hasNext;
    bookmarks = [...bookmarksList, bookmarks];
  }

  onMount(() => {
    const { bookmarks: bookmarksList, hasNext } = nextPage(currentPage);

    hasNextPage = hasNext;
    bookmarks = [bookmarksList];
  });

  $: {
    switch ($locale) {
      case 'en':
        title = 'Esteban Borai | Bookmarks';
        description =
          'Bookmarks from relevant articles, blog posts, and documentation.';
        break;
      case 'es':
        title = 'Esteban Borai | Marcadores';
        description =
          'Marcadores de artículos, publicaciones de blog y documentación relevantes.';
        break;
      case 'hu':
        title = 'Borai Esteban | Könyvjelzők';
        description =
          'Könyvjelzők a releváns cikkekből, blogbejegyzésekből és dokumentációból.';
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

<section class="mx-auto p-4 md:p-2 md:w-11/12">
  <ul>
    {#each bookmarks as page}
      <ul
        class="flex flex-col md:grid md:grid-cols-3 md:gap-6 px-4 md:px-0 max-w-[1080px] mx-auto"
      >
        {#each page as { title, description, url, tags, previewImageUrl, createdAt }}
          <Bookmark
            {title}
            {url}
            {description}
            {tags}
            {previewImageUrl}
            createdAt={new Date(createdAt)}
          />
        {/each}
      </ul>
    {/each}
  </ul>
  {#if hasNextPage}
    <div class="flex justify-center items-center py-6">
      <Button
        title="Show more"
        disabled={!hasNextPage}
        on:click={handleShowMore}
      />
    </div>
  {/if}
</section>
