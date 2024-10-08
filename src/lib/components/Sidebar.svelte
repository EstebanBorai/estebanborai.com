<script lang="ts">
  import { onMount } from 'svelte';

  import LL, { setLocale, locale } from '$i18n/i18n-svelte';
  import Home from '~icons/custom/home';
  import Repo from '~icons/custom/repo';
  import { page } from '$app/stores';
  import GitHub from '$lib/components/icons/GitHub.svelte';
  import LinkedIn from '$lib/components/icons/LinkedIn.svelte';
  import StackOverflow from '$lib/components/icons/StackOverflow.svelte';
  import Mastodon from '$lib/components/icons/Mastodon.svelte';
  import Reddit from '$lib/components/icons/Reddit.svelte';
  import { replaceLocaleInUrl } from '$lib/utils/locale';

  import type { Locales } from '$i18n/i18n-types';

  const LINKS = [
    {
      icon: Repo,
      href: `/${$page.params.lang}/notes`,
      text: $LL.LAYOUT.NAV.NOTES(),
    },
    {
      icon: Repo,
      href: `/${$page.params.lang}/bookmarks`,
      text: $LL.LAYOUT.NAV.BOOKMARKS(),
    },
  ];

  const LANGS: {
    text: string;
    locale: Locales;
  }[] = [
    {
      text: 'Español',
      locale: 'es',
    },
    {
      text: 'English',
      locale: 'en',
    },
    {
      text: 'Magyar',
      locale: 'hu',
    },
  ];

  let currentTheme: 'dark' | 'light' = 'light';

  onMount(() => {
    currentTheme =
      localStorage.theme === 'dark' ||
      (!('theme' in localStorage) &&
        window.matchMedia('(prefers-color-scheme: dark)').matches)
        ? 'dark'
        : 'light';

    if (currentTheme === 'dark') {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.theme = 'light';
    }
  });

  function changeTheme(theme: 'light' | 'dark'): void {
    const removeClass = theme === 'light' ? 'dark' : 'light';

    document.documentElement.classList.remove(removeClass);
    document.documentElement.classList.add(theme);
    localStorage.theme = theme;
  }

  function changeLanguage(locale: Locales): void {
    const lang = $page.params.lang;

    if (lang === locale) {
      return;
    }

    const next = replaceLocaleInUrl($page.url, locale);
    setLocale(locale);
    window.location.href = next;
  }

  const handleLanguageChange = (ev: Event) => {
    const lang = (ev.target as unknown as { value: string }).value as Locales;
    changeLanguage(lang);
  };

  const handleThemeChange = (ev: Event) => {
    const theme = (ev.target as unknown as { value: string }).value as
      | 'dark'
      | 'light';
    changeTheme(theme);
  };
</script>

<aside
  class="hidden md:flex bg-lt-base-alt dark:bg-dk-base-alt px-4 pt-8 flex-col justify-between"
>
  <div>
    <header class="flex flex-col justify-center items-center">
      <figure class="overflow-hidden h-[190px] w-[190px] rounded-full">
        <img
          src="https://avatars.githubusercontent.com/u/34756077?v=4"
          alt="Esteban Borai Profile"
          height="190"
          width="190"
        />
      </figure>
      <article class="flex flex-col justify-center items-center py-6">
        <h1 class="text-3xl font-semibold">Esteban Borai</h1>
        <p class="text-gray-800 dark:text-gray-400 text-center py-2">
          {@html $LL.HOMEPAGE.ABOUT()}
        </p>
      </article>
    </header>
    <nav>
      <ul class="flex flex-col space-y-2">
        {#each LINKS as { href, text, icon }}
          <li class="py-2 px-4 bg-zinc-800 rounded-md">
            <a class="flex items-center justify-start" {href}>
              <figure class="mr-2">
                <svelte:component this={icon} class="w-4 h-4" />
              </figure>
              {text}
            </a>
          </li>
        {/each}
      </ul>
    </nav>
  </div>
</aside>
