<script lang="ts">
  let data: HTMLElement = null;
  let href = '';
  let id = '';

  $: {
    if (typeof data?.innerText === 'string') {
      id = data?.innerText?.toLowerCase().trim().split(' ').join('-');
      href = `#${id}`;
    }
  }

  const focusElement = () => {
    data.scrollIntoView();
  };
</script>

<h3 {id} class="text-xl font-display relative group py-4" bind:this={data}>
  <a class="flex items-center no-underline" {href} on:click={focusElement}>
    <span class="mr-2">
      <slot />
    </span>
    <span
      class="h-8 w-8 invisible flex justify-center items-center group-hover:visible bg-gray-200 font-bold rounded-full dark:bg-gray-600 dark:text-gray-200"
      >⛺︎
    </span>
  </a>
</h3>
