<script lang="ts">
  import { page } from '$app/stores';
  import { locale } from '$i18n/i18n-svelte';
  import { humanDate } from '$lib/utils/date';

  export let title: string;
  export let description: string;
  export let publishDate: Date;
  export let tags: string[];
  export let slug: string;
  export let previewImageUrl: string;

  const LANGUAGE_COLOR: {
    [lang: string]: string;
  } = {
    python: '#3572A5',
    rust: '#dea584',
    svelte: '#ff3e00',
    typescript: '#2b7489',
  };
  const langTags = Object.keys(LANGUAGE_COLOR);

  let formattedDate = humanDate($locale, publishDate);
  let displayTags = tags
    .sort((a, b) => (langTags.includes(a) ? 0 : 1))
    .slice(0, 3);
</script>

<li
  class="mb-4 md:mb-0 last-of-type:mb-0 border-b border-gray-600 md:border-0 last-of-type:border-0 mb-8"
>
  <figure
    class="rounded overflow-hidden pb-4 flex justify-center items-center overflow-hidden h-[150px]"
  >
    <img loading="lazy" decoding="async" alt={title} src={previewImageUrl} />
  </figure>
  <header>
    <h3
      class="text-xl py-4 text-gray-200 font-extrabold cursor-pointer hover:text-link hover:underline"
    >
      <a href="/{$page.params.lang}/notes/{slug}">
        {title}
      </a>
    </h3>
  </header>
  <main>
    <p aria-label={description} class="line-clamp-3 text-gray-400">
      {description}
    </p>
    <ul class="flex justify-start items-start flex-wrap gap-2 py-2">
      {#each displayTags as category}
        <span
          class="border border-gray-400 text-xs text-gray-400 py-1 px-4 rounded-full text-center uppercase"
        >
          {#if category.toLowerCase() in LANGUAGE_COLOR}
            <span
              class="inline-block mr-1 rounded-full h-2 w-2"
              style="background-color: {LANGUAGE_COLOR[
                category.toLowerCase()
              ]};"
            />
          {/if}
          {category}
        </span>
      {/each}
    </ul>
  </main>
  <footer class="flex flex-col">
    <div class="flex">
      <span class="flex items-center">
        <time
          class="py-2 text-gray-600 text-sm uppercase"
          datetime={publishDate.toString()}
        >
          {formattedDate}
        </time>
      </span>
    </div>
  </footer>
</li>
