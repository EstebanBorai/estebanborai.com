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

<footer
  class="text-sm flex flex-col border-t py-6 px-4 border-lt-alte dark:border-dk-alte"
>
  <div class="flex flex-col md:grid md:grid-cols-12">
    <article class="md:col-start-2 md:col-span-3">
      <h4 class="uppercase text-xs">{$LL.HOMEPAGE.SOCIAL_LINKS()}</h4>
      <ul class="flex flex-col justify-evenly py-4 space-y-2 text-xs">
        <li class="hover:text-zinc-900 text-zinc-600">
          <a
            class="flex items-center"
            rel="noopener noreferrer"
            href="https://www.github.com/estebanborai"
            target="_blank"
          >
            <GitHub class="w-6 h-6" />
            <span class="pl-2"> GitHub </span>
          </a>
        </li>
        <li class="hover:text-zinc-900 text-zinc-600">
          <a
            class="flex items-center"
            rel="noopener noreferrer"
            href="https://www.reddit.com/user/estebanborai"
            target="_blank"
          >
            <Reddit class="w-6 h-6" />
            <span class="pl-2"> Reddit </span>
          </a>
        </li>
      </ul>
    </article>
    <article class="md:col-start-6 md:col-span-3">
      <h4 class="uppercase text-xs">SiteMap</h4>
      <ul class="flex flex-col justify-evenly py-4 text-xs">
        <li class="mr-4">
          <a href="/{$page.params.lang}/notes">
            {$LL.LAYOUT.NAV.NOTES()}
          </a>
        </li>
        <li class="mr-4">
          <a href="/{$page.params.lang}/bookmarks">
            {$LL.LAYOUT.NAV.BOOKMARKS()}
          </a>
        </li>
      </ul>
    </article>
  </div>
  <small class="block text-center text-xs"
    >{$LL.LAYOUT.FOOTER.COPYRGHT({
      start: 2017,
      end: new Date().getFullYear(),
    })}</small
  >
  <a
    class="block text-center text-xs pt-2"
    href="https://github.com/EstebanBorai/estebanborai.com"
    target="_blank">{$LL.LAYOUT.FOOTER.SOURCE_CODE()}</a
  >
</footer>
