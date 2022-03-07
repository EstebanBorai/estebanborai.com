<script lang="ts">
  import { onMount } from 'svelte';

  import Moon from '$lib/components/icons/Moon.svelte';
  import Sun from '$lib/components/icons/Sun.svelte';

  let useDarkMode = false;

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
</script>

<header
  class="flex justify-center px-4 h-[70px] bg-light-background dark:bg-dark-background"
>
  <div class="safe-zone flex items-center justify-between">
    <h1>Esteban Borai</h1>
    <nav class="flex items-center">
      <ul class="flex pr-4">
        <li class="mx-2 font-semibold">
          <a href="/"> Home </a>
        </li>
        <li class="mx-2 font-semibold">
          <a href="/notes"> Notes </a>
        </li>
      </ul>
      <button
        class="flex justify-center items-center h-5 w-5"
        on:click={toggleDarkMode}
      >
        {#if useDarkMode}
          <span>
            <Sun className="h-5 w-5" />
          </span>
        {:else}
          <span>
            <Moon className="h-5 w-5" />
          </span>
        {/if}
      </button>
    </nav>
  </div>
</header>
