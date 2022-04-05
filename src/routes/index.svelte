<script lang="ts">
  import GitHub from '$lib/components/icons/GitHub.svelte';
  import Itchio from '$lib/components/icons/Itchio.svelte';
  import LinkedIn from '$lib/components/icons/LinkedIn.svelte';
  import StackOverflow from '$lib/components/icons/StackOverflow.svelte';
  import Twitter from '$lib/components/icons/Twitter.svelte';

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
  class="container flex flex-col justify-center items-center md:flex-row md:mx-auto md:justify-evenly md:items-center"
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
    <strong class="text-3xl md:text-4xl py-4 text-center md:text-left"
      >Hi I'm Esteban Borai,</strong
    >
    <p class="text-center md:text-left">
      Software Developer. Using Rust 🦀 and TypeScript 👨🏻‍💻 on a daily basis.
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

<style>
  .container {
    min-height: calc(100vh - 70px);
  }
</style>
