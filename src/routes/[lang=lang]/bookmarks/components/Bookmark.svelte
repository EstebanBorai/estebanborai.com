<script lang="ts">
  import { locale } from '$i18n/i18n-svelte';
  import Tag from '$lib/components/Tag.svelte';
  import { humanDate } from '$lib/utils/date';

  export let title: string;
  export let url: string;
  export let tags: string[];
  export let description: string;
  export let previewImageUrl: string;
  export let createdAt: Date;

  let formattedDate = humanDate($locale, new Date(createdAt));
  let displayTags = tags.slice(0, 3);
</script>

<li
  class="md:mb-0 last-of-type:mb-0 border-b border-gray-600 md:border-0 last-of-type:border-0 mb-8 h-[400px]"
>
  <figure
    class="rounded pb-4 flex justify-center items-start overflow-hidden h-[150px]"
  >
    <img loading="lazy" decoding="async" alt={title} src={previewImageUrl} />
  </figure>
  <header>
    <h3
      class="text-xl py-4 text-gray-200 font-extrabold cursor-pointer hover:text-link hover:underline"
    >
      <a href={url}>
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
        <Tag title={category} />
      {/each}
    </ul>
  </main>
  <footer class="flex flex-col">
    <div class="flex">
      <span class="flex items-center">
        <time
          class="py-2 text-gray-600 text-xs uppercase"
          datetime={createdAt.toString()}
        >
          {formattedDate}
        </time>
      </span>
    </div>
  </footer>
</li>
