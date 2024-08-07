<script lang="ts">
  import { page } from '$app/stores';
  import { locale } from '$i18n/i18n-svelte';
  import Tag from '$lib/components/Tag.svelte';
  import { humanDate } from '$lib/utils/date';

  export let title: string;
  export let description: string;
  export let publishDate: Date;
  export let tags: string[];
  export let slug: string;
  export let previewImageUrl: string;

  const langTags = ['python', 'rust', 'svelte', 'typescript'];

  let formattedDate = humanDate($locale, publishDate);
  let displayTags = tags
    .sort((a, b) => (langTags.includes(a) ? 0 : 1))
    .slice(0, 3);
</script>

<li
  class="grid gap-4 grid-cols-[100px,auto] md:grid-cols-[200px,auto] md:w-[720px] w-full"
>
  <figure
    class="rounded flex justify-center items-center overflow-hidden w-[100px] h-[100px] md:h-[200px] md:w-[200px]"
  >
    <img
      class="w-full h-full object-cover"
      loading="lazy"
      decoding="async"
      alt={title}
      src={previewImageUrl}
    />
  </figure>
  <article>
    <h3 class="text-lg md:text-xl hover:underline">
      <a href="/{$page.params.lang}/notes/{slug}">
        {title}
      </a>
    </h3>
    <p aria-label={description} class="py-2 text-sm line-clamp-3 text-gray-400">
      {description}
    </p>
    <ul class="flex justify-start items-start flex-wrap gap-2 py-2">
      {#each displayTags as category}
        <Tag title={category} />
      {/each}
    </ul>
    <div class="flex">
      <span class="flex items-center">
        <time
          class="py-2 text-gray-600 text-xs uppercase"
          datetime={publishDate.toString()}
        >
          {formattedDate}
        </time>
      </span>
    </div>
  </article>
</li>
