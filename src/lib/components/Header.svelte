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

<header
  class="flex justify-center px-4 h-[70px] bg-light-background dark:bg-dark-background"
>
  <div class="safe-zone flex items-center justify-between">
    <a href="/{$page.params.lang}">
      <h1>Esteban Borai</h1>
    </a>
    <nav class="flex items-center">
      <ul class="flex pr-4">
        <li class="mx-2 font-semibold">
          <a href="/{$page.params.lang}"> {$LL.LAYOUT.NAV.HOME()} </a>
        </li>
        <li class="mx-2 font-semibold">
          <a href="/{$page.params.lang}/notes"> {$LL.LAYOUT.NAV.NOTES()} </a>
        </li>
      </ul>
      <div class="flex justify-center items-center space-x-4">
        <figure class="h-6 w-6">
          <button class="h-6 w-6" on:click={toggleLanguageMenu}>
            <Translate class="h-6 w-6" />
          </button>
          {#if isLangMenuOpen}
            <div class="relative">
              <ul
                class="absolute flex flex-col space-y-2 p-2 border-md border-bg-light-background-alt dark:border-bg-dark-background-alt bg-light-background dark:bg-dark-background rounded-lg h-[150px] w-[100px]"
              >
                <li class="lang-opt">
                  <button
                    class:lang-active={$page.params.lang === 'en'}
                    on:click={() => changeLanguage('en')}
                  >
                    🇺🇸&nbsp;English
                  </button>
                </li>
                <li class="lang-opt">
                  <button
                    class:lang-active={$page.params.lang === 'es'}
                    on:click={() => changeLanguage('es')}
                  >
                    🇪🇸&nbsp;Español
                  </button>
                </li>
                <li class="lang-opt border-none">
                  <button
                    class:lang-active={$page.params.lang === 'hu'}
                    on:click={() => changeLanguage('hu')}
                  >
                    🇭🇺&nbsp;Magyar
                  </button>
                </li>
              </ul>
            </div>
          {/if}
        </figure>
        <button class="h-5 w-5" on:click={toggleDarkMode}>
          {#if useDarkMode}
            <span>
              <Sun class="h-5 w-5" />
            </span>
          {:else}
            <span>
              <Moon class="h-5 w-5" />
            </span>
          {/if}
        </button>
      </div>
    </nav>
  </div>
</header>

<style>
  .lang-opt {
    @apply py-2 border-b border-gray-400 cursor-pointer;
  }

  .lang-active {
    @apply font-bold;
  }
</style>
