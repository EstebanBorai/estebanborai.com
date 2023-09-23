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

<section class="flex flex-col items-center h-screen">
  <div class="w-full p-4 md:p-2 md:w-11/12 max-w-[1080px] mx-auto">
    <div>
      <div class="grid grid-cols-1 gap-6 md:grid-cols-4">
        <div class="min-h-fit col-span-1 md:col-span-2">
          <p class="text-xl py-2">
            {@html $LL.HOMEPAGE.HI({ name: 'Esteban', surname: 'Borai' })}
          </p>
          <p class="text-4xl">
            {@html $LL.HOMEPAGE.ABOUT()}
          </p>
        </div>
        <div class="col-span-1 md:col-span-2">
          <article class="flex flex-col">
            <div>
              <h3 class="text-xl font-semibold pb-2">
                {@html $LL.HOMEPAGE.LATEST_NOTES()}
              </h3>
              <hr class="text-gray-600 pb-2" />
              <div class="flex flex-col pb-6 md:flex-row">
                <div class="flex flex-col justify-center">
                  <a
                    href="/{$page.params.lang}/notes/{data.notesIndex[0].slug}"
                    class="text-xl font-bold py-2 text-gray-400 hover:text-gray-200"
                  >
                    {data.notesIndex[0].meta.title}
                  </a>
                  <p class="text-gray-400">
                    {data.notesIndex[0].meta.description}
                  </p>
                  <ul class="space-x-2 py-2">
                    {#each data.notesIndex[0].meta.categories as category}
                      <Tag title={category} />
                    {/each}
                  </ul>
                  <time
                    class="py-2 text-gray-600 text-sm uppercase"
                    datetime={data.notesIndex[0].meta.date}
                  >
                    {humanDate($locale, new Date(data.notesIndex[0].meta.date))}
                  </time>
                </div>
              </div>
              <h3 class="text-xl font-semibold pb-2">
                {@html $LL.HOMEPAGE.SOCIAL_LINKS()}
              </h3>
              <hr class="pb-2 text-gray-600" />
              <ul class="flex py-4 mx-auto md:mx-0">
                <li class="mr-4">
                  <a
                    rel="noopener noreferrer"
                    href="https://github.com/EstebanBorai"
                    target="_blank"
                  >
                    <GitHub
                      class="hover:text-gray-200 text-gray-400 w-6 h-6 fill-current"
                    />
                  </a>
                </li>
                <li class="mr-4">
                  <a
                    rel="noopener noreferrer"
                    href="https://stackoverflow.com/users/9888500/esteban-borai"
                    target="_blank"
                  >
                    <StackOverflow
                      class="hover:text-gray-200 text-gray-400 w-6 h-6"
                    />
                  </a>
                </li>
                <li class="mr-4">
                  <a
                    rel="noopener noreferrer"
                    href="https://www.linkedin.com/in/esteban-b-241ba0135/"
                    target="_blank"
                  >
                    <LinkedIn
                      class="hover:text-gray-200 text-gray-400 w-6 h-6"
                    />
                  </a>
                </li>
                <li class="mr-4">
                  <a
                    rel="me noopener noreferrer"
                    href="https://hachyderm.io/@EstebanBorai"
                    target="_blank"
                  >
                    <Mastodon
                      class="hover:text-gray-200 text-gray-400 w-6 h-6"
                    />
                  </a>
                </li>
                <li class="mr-4">
                  <a
                    rel="noopener noreferrer"
                    href="https://www.reddit.com/user/estebanborai"
                    target="_blank"
                  >
                    <Reddit class="hover:text-gray-200 text-gray-400 w-6 h-6" />
                  </a>
                </li>
              </ul>
            </div>
          </article>
        </div>
      </div>
    </div>
  </div>
</section>
