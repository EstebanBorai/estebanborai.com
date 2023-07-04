<script lang="ts">
  import { humanDate } from '$lib/utils/date';

  import LL, { locale } from '$i18n/i18n-svelte';
  import GitHub from '$lib/components/icons/GitHub.svelte';
  import Itchio from '$lib/components/icons/Itchio.svelte';
  import LinkedIn from '$lib/components/icons/LinkedIn.svelte';
  import StackOverflow from '$lib/components/icons/StackOverflow.svelte';
  import Twitter from '$lib/components/icons/Twitter.svelte';
  import { page } from '$app/stores';

  export let data: {
    notes: Domain.BlogNote[];
  };

  let title = 'Esteban Borai | Software Developer';
  let description =
    'A Software Developer interested in Systems Programming and Web Development.';
  let avatarUrl = 'https://avatars.githubusercontent.com/u/34756077?v=4';
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

<section
  class="flex flex-col justify-center items-center md:flex-row md:mx-auto md:justify-evenly md:items-center"
>
  <div class="flex md:items-center md:justify-center w-1/2 py-4 w-[300px]">
    <figure class="w-[300px]">
      <img
        class="rounded-full"
        src={avatarUrl}
        height="300"
        width="300"
        alt="A selfie"
      />
    </figure>
  </div>
  <article class="flex flex-col py-4 md:w-3/6 w-4/6">
    <strong class="font-body text-3xl md:text-4xl py-4 text-center md:text-left"
      >{$LL.HOMEPAGE.HI({ name: 'Esteban', surname: 'Borai' })}</strong
    >
    <p class="text-center md:text-left">
      {$LL.HOMEPAGE.ABOUT()}
    </p>
    <ul class="flex py-4 mx-auto md:mx-0">
      <li class="mr-4">
        <a
          rel="noopener noreferrer"
          href="https://stackoverflow.com/users/9888500/esteban-borai"
          target="_blank"
        >
          <StackOverflow class="w-6 h-6" />
        </a>
      </li>
      <li class="mr-4">
        <a
          rel="noopener noreferrer"
          href="https://github.com/EstebanBorai"
          target="_blank"
        >
          <GitHub class="w-6 h-6 fill-current" />
        </a>
      </li>
      <li class="mr-4">
        <a
          rel="noopener noreferrer"
          href="https://www.linkedin.com/in/esteban-b-241ba0135/"
          target="_blank"
        >
          <LinkedIn class="w-6 h-6" />
        </a>
      </li>
      <li class="mr-4">
        <a
          rel="noopener noreferrer"
          href="https://twitter.com/EstebanBorai"
          target="_blank"
        >
          <Twitter class="w-6 h-6" />
        </a>
      </li>
      <li class="mr-4">
        <a
          rel="noopener noreferrer"
          href="https://estebanborai.itch.io"
          target="_blank"
        >
          <Itchio class="w-6 h-6" />
        </a>
      </li>
    </ul>
  </article>
</section>
<section class="my-4">
  <div class="max-w-1/2">
    <h2 class="font-body text-xl mb-4">{$LL.HOMEPAGE.LATEST_NOTES()}</h2>
    <ul>
      {#each data.notes as note}
        <li>
          <a
            href="/{$page.params.lang}/notes/{note.slug}"
            class="py-2 px-4 flex items-center justify-between rounded hover:bg-light-background dark:hover:bg-dark-background cursor-pointer flex items-center mb-4 last-of-type:mb-0"
          >
            <div class="flex items-center">
              <figure class="mr-4 w-4 h-4">
                <img src={`/images/icons/${note.icon}.png`} alt={note.icon} />
              </figure>
              <span>
                {note.title}
              </span>
            </div>
            <span class="flex items-center mr-2">
              <figure class="mr-2">
                <!-- <Calendar size={16} /> -->
              </figure>
              <time
                class="text-sm mr-2"
                datetime={new Date(note.date).toString()}
                >{humanDate($locale, new Date(note.date))}</time
              >
            </span>
          </a>
        </li>
      {/each}
    </ul>
  </div>
</section>
