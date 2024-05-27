<script lang="ts">
  import { humanDate } from '$lib/utils/date';

  import LL, { locale } from '$i18n/i18n-svelte';
  import GitHub from '$lib/components/icons/GitHub.svelte';
  import LinkedIn from '$lib/components/icons/LinkedIn.svelte';
  import StackOverflow from '$lib/components/icons/StackOverflow.svelte';
  import { page } from '$app/stores';
  import Mastodon from '$lib/components/icons/Mastodon.svelte';
  import Reddit from '$lib/components/icons/Reddit.svelte';
  import Tag from '$lib/components/Tag.svelte';

  export let data: {
    notesIndex: Domain.BlogNote[];
  };

  let title = 'Esteban Borai | Software Developer';
  let description =
    'A Software Developer interested in Systems Programming and Web Development.';
  let avatarUrl = 'https://avatars.githubusercontent.com/u/34756077?v=4';

  $: {
    switch ($locale) {
      case 'en':
        title = 'Esteban Borai | Software Developer';
        description =
          'A Software Developer interested in Systems Programming and Web Development.';
        break;
      case 'es':
        title = 'Esteban Borai | Desarrollador de Software';
        description =
          'Un Desarrollador de Software interesado en Programación de Sistemas y Desarrollo Web.';
        break;
      case 'hu':
        title = 'Borai Esteban | Szoftverfejlesztő';
        description =
          'Egy Szoftverfejlesztő, aki érdeklődik a Rendszerprogramozás és a Webfejlesztés iránt.';
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
