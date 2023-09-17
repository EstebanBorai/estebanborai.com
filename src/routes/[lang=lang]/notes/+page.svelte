<script lang="ts">
  import { locale } from '$i18n/i18n-svelte';
  import Entry from './components/Entry.svelte';

  export let data: {
    notesIndex: Domain.BlogNote[];
  };

  let title = 'Esteban Borai | Notes';
  let description =
    'Notes taken while reading about computer science and software development.';
  let avatarUrl = 'https://avatars.githubusercontent.com/u/34756077?v=4';

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

<section class="mx-auto p-4 md:p-2 md:w-11/12">
  <ul
    class="flex flex-col md:grid md:grid-cols-3 md:gap-6 px-4 md:px-0 max-w-[1080px] mx-auto"
  >
    {#each data.notesIndex as { meta, slug }}
      <Entry
        title={meta.title}
        description={meta.description}
        publishDate={new Date(meta.date)}
        tags={meta.categories}
        {slug}
        previewImageUrl={meta.preview_image_url}
      />
    {/each}
  </ul>
</section>
