<script lang="ts" context="module">
  import a from './custom/a.svelte';
  import blockquote from './custom/blockquote.svelte';
  import h2 from './custom/h2.svelte';
  import h3 from './custom/h3.svelte';
  import img from './custom/img.svelte';
  import table from './custom/table.svelte';
  import td from './custom/td.svelte';
  import th from './custom/th.svelte';

  export { a, blockquote, h2, h3, img, table, th, td };
</script>

<script lang="ts">
  import { locale } from '$i18n/i18n-svelte';
  import { humanDate } from '$lib/utils/date';

  import './styles/note.css';
  import './styles/prism-one-dark.css';

  export let title;
  export let description;
  export let date;
  export let preview_image_url;

  const publishedTime = new Date(date);
  const formattedDate = humanDate($locale, publishedTime);
</script>

<svelte:head>
  <title>{title} | Esteban Borai</title>
  <meta name="description" content={description} />
  <!-- Schema.org markup for Google+ -->
  <meta itemprop="name" content={title} />
  <meta itemprop="description" content={description} />
  <meta itemprop="image" content={preview_image_url} />
  <!-- Open Graph data -->
  <meta property="og:title" content={title} />
  <meta property="og:type" content="article" />
  <meta property="og:url" content="https://estebanborai.com/" />
  <meta property="og:image" content={preview_image_url} />
  <meta property="og:description" content={description} />
  <meta property="og:site_name" content="Esteban Borai" />
  <meta property="article:published_time" content={publishedTime.toJSON()} />
  <meta property="article:modified_time" content={publishedTime.toJSON()} />
  <!-- Twitter Card data -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:site" content="@EstebanBorai" />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:creator" content="@EstebanBorai" />
  <meta name="twitter:image:src" content={preview_image_url} />
</svelte:head>

<div class="w-full md:max-w-[1000px] mx-auto pb-6">
  <figure
    class="flex justify-center items-center overflow-hidden rounded h-[300px]"
  >
    <img class="w-full" src={preview_image_url} alt={title} />
  </figure>
  <header class="py-4">
    <h1 class="text-3xl py-4 font-semibold">{title}</h1>
    <div class="flex mb-2">
      <span class="flex items-center mr-2">
        <time
          class="text-sm mr-2"
          datetime={Intl.DateTimeFormat($locale).format(publishedTime)}
          >{formattedDate}</time
        >
      </span>
    </div>
  </header>
  <article>
    <slot />
  </article>
</div>
