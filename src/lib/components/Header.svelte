<script lang="ts">
  import { onMount } from 'svelte';

  import LL, { setLocale } from '$i18n/i18n-svelte';
  import Globe from '~icons/custom/globe';
  import Home from '~icons/custom/home';
  import Moon from '~icons/custom/moon';
  import Repo from '~icons/custom/repo';
  import Sun from '~icons/custom/sun';
  import { page } from '$app/stores';
  import { replaceLocaleInUrl } from '$lib/utils/locale';

  import type { Locales } from '$i18n/i18n-types';

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
      <div class="flex items-center space-x-4">
        <nav>
          <ul class="flex items-center space-x-4">
            {#each LINKS as { href, text, icon }}
              <li>
                <a class="flex items-center justify-center" {href}>
                  <figure class="mr-2">
                    <svelte:component this={icon} class="w-4 h-4" />
                  </figure>
                  {text}
                </a>
              </li>
            {/each}
          </ul>
        </nav>
        <ul class="flex items-center space-x-2">
          <li>
            <button
              class="p-2 rounded-md"
              type="button"
              on:click={toggleDarkMode}
            >
              {#if useDarkMode}
                <Sun class="w-5 h-5" />
              {:else}
                <Moon class="w-5 h-5" />
              {/if}
            </button>
          </li>
          <li>
            <button
              class="p-2 rounded-md"
              type="button"
              on:click={openLanguageMenu}
            >
              <Globe class="w-5 h-5" />
            </button>
          </li>
        </ul>
      </div>
    </div>
  </div>
</header>

{#if isLangMenuOpen}
  <div
    class="bg-gray-600/80 flex justify-center items-center h-screen w-screen fixed top-0 left-0"
  >
    <div class="text-white bg-gray-800 rounded-md p-4 w-[300px] space-y-4">
      <span class="block text-center text-lg">Choose language</span>
      <ul class="space-y-4">
        {#each LANGS as { locale, text }}
          <li>
            <button
              class="p-4 hover:bg-gray-400 bg-gray-600 w-full rounded-md"
              type="button"
              on:click={() => changeLanguage(locale)}
            >
              {text}
            </button>
          </li>
        {/each}
      </ul>
    </div>
  </div>
{/if}
