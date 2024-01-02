<script lang="ts">
  import LL, { setLocale } from '$i18n/i18n-svelte';
  // import Globe from '~icons/custom/globe';
  import Home from '~icons/custom/home';
  // import Moon from '~icons/custom/moon';
  import Repo from '~icons/custom/repo';
  // import Sun from '~icons/custom/sun';
  import { page } from '$app/stores';
  import GitHub from '$lib/components/icons/GitHub.svelte';
  import LinkedIn from '$lib/components/icons/LinkedIn.svelte';
  import StackOverflow from '$lib/components/icons/StackOverflow.svelte';
  import Mastodon from '$lib/components/icons/Mastodon.svelte';
  import Reddit from '$lib/components/icons/Reddit.svelte';
  import { replaceLocaleInUrl } from '$lib/utils/locale';

  import type { Locales } from '$i18n/i18n-types';
  import { onMount } from 'svelte';

  const LINKS = [
    {
      icon: Home,
      text: $LL.LAYOUT.NAV.HOME(),
      href: `/${$page.params.lang}`,
    },
    {
      icon: Repo,
      href: `/${$page.params.lang}/notes`,
      text: $LL.LAYOUT.NAV.NOTES(),
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

  let useDarkMode = false;
  let isLangMenuOpen = false;

  onMount(() => {
    useDarkMode =
      localStorage.theme === 'dark' ||
      (!('theme' in localStorage) &&
        window.matchMedia('(prefers-color-scheme: dark)').matches);

    if (useDarkMode) {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.theme = 'light';
    }
  });

  function toggleDarkMode(): void {
    if (useDarkMode) {
      document.documentElement.classList.remove('dark');
      localStorage.theme = 'light';
      useDarkMode = false;
    } else {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
      useDarkMode = true;
    }
  }

  function openLanguageMenu(): void {
    isLangMenuOpen = true;
  }

  function closeLanguageMenu(): void {
    isLangMenuOpen = false;
  }

  function changeLanguage(locale: Locales): void {
    const lang = $page.params.lang;

    if (lang === locale) {
      isLangMenuOpen = false;
      return;
    }

    const next = replaceLocaleInUrl($page.url, locale);
    setLocale(locale);
    closeLanguageMenu();
    window.location.href = next;
  }
</script>

<aside
  class="hidden md:flex bg-light-background-alt dark:bg-dark-background-alt px-4 pt-8 flex-col justify-between"
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
        <h1 class="text-lg">Esteban Borai</h1>
        <p class="text-gray-800 dark:text-gray-400 text-center py-2">
          {@html $LL.HOMEPAGE.ABOUT()}
        </p>
      </article>
    </header>
    <nav>
      <ul class="flex flex-col space-y-2">
        {#each LINKS as { href, text, icon }}
          <li class="py-2 px-4 bg-red-200 rounded-md">
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
  <div class="pb-4">
    <footer>
      <ul class="flex justify-evenly py-4">
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
            <StackOverflow class="hover:text-gray-200 text-gray-400 w-6 h-6" />
          </a>
        </li>
        <li class="mr-4">
          <a
            rel="noopener noreferrer"
            href="https://www.linkedin.com/in/esteban-b-241ba0135/"
            target="_blank"
          >
            <LinkedIn class="hover:text-gray-200 text-gray-400 w-6 h-6" />
          </a>
        </li>
        <li class="mr-4">
          <a
            rel="me noopener noreferrer"
            href="https://hachyderm.io/@EstebanBorai"
            target="_blank"
          >
            <Mastodon class="hover:text-gray-200 text-gray-400 w-6 h-6" />
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
    </footer>
    <select name="" id="">
      {#each LANGS as { text, locale }}
        <option value={locale}>{text}</option>
      {/each}
    </select>
    <select name="" id="">
      <option value="">Dark</option>
      <option value="">Light</option>
    </select>
  </div>
</aside>
