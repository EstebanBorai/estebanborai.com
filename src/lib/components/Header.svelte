<script lang="ts">
  import { onMount } from 'svelte';

  import { page } from '$app/stores';
  import LL, { setLocale, locale } from '$i18n/i18n-svelte';
  import { replaceLocaleInUrl } from '$lib/utils/locale';

  import type { Locales } from '$i18n/i18n-types';

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

<header class="border-b border-lt-alte dark:border-dk-alte">
  <div
    class="mx-auto text-sm flex justify-between items-center py-2 px-4 max-width"
  >
    <div class="flex justify-between items-center">
      <a href="/{$page.params.lang}">
        <h1 class="m-0 p-0">Esteban Borai</h1>
      </a>
      <nav class="hidden md:flex justify-center items-center px-4 md:space-x-2">
        <a href="/{$page.params.lang}/notes">
          {$LL.LAYOUT.NAV.NOTES()}
        </a>
        <a href="/{$page.params.lang}/bookmarks">
          {$LL.LAYOUT.NAV.BOOKMARKS()}
        </a>
      </nav>
    </div>
    <div class="hidden md:flex items-center justify-center">
      <select class="text-sm" value={$locale} on:change={handleLanguageChange}>
        {#each LANGS as { text, locale }}
          <option value={locale}>{text}</option>
        {/each}
      </select>
      <select
        class="text-sm"
        value={currentTheme}
        on:change={handleThemeChange}
      >
        <option value="dark">Dark</option>
        <option value="light">Light</option>
      </select>
    </div>
    <button class="flex md:hidden"> M </button>
  </div>
</header>
