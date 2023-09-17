<script lang="ts">
  import { onMount } from 'svelte';

  import LL, { setLocale } from '$i18n/i18n-svelte';
  import Translate from '$lib/components/icons/Translate.svelte';
  import Moon from '$lib/components/icons/Moon.svelte';
  import Sun from '$lib/components/icons/Sun.svelte';
  import { page } from '$app/stores';
  import { replaceLocaleInUrl } from '$lib/utils/locale';

  import type { Locales } from '$i18n/i18n-types';

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

  function toggleLanguageMenu(): void {
    isLangMenuOpen = !isLangMenuOpen;
  }

  function changeLanguage(locale: Locales): void {
    const lang = $page.params.lang;

    if (lang === locale) {
      isLangMenuOpen = false;
      return;
    }

    const next = replaceLocaleInUrl($page.url, locale);
    setLocale(locale);
    window.location.href = next;
  }
</script>

<header class="flex justify-center items-center">
  <div class="w-full p-4 md:px-2 md:w-11/12">
    <div class="flex items-center justify-between">
      <a href="/">
        <img
          alt="Esteban Borai Memoji"
          loading="lazy"
          decoding="async"
          height="36"
          width="36"
          src="/images/png/memoji_dev.png"
          style="color: transparent;"
        />
      </a>
      <nav>
        <ul class="flex items-center space-x-4">
          <li>
            <a href="/{$page.params.lang}"> {$LL.LAYOUT.NAV.HOME()} </a>
          </li>
          <li>
            <a href="/{$page.params.lang}/notes"> {$LL.LAYOUT.NAV.NOTES()} </a>
          </li>
        </ul>
      </nav>
    </div>
  </div>
</header>
