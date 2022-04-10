<script lang="ts">
  let data: HTMLElement = null;
  let href = '';
  let id = '';

  $: {
    if (typeof data?.innerText === 'string') {
      id = data?.innerText?.toLowerCase().split(' ').join('-');
      href = `#${id}`;
    }
  }

  const focusElement = () => {
    data.scrollIntoView();
  };
</script>

<h2 {id} class="relative group" bind:this={data}>
  <a class="no-underline" {href} on:click={focusElement}>
    <span
      class="invisible group-hover:visible absolute left-[-2.5rem] flex items-center justify-center bg-gray-200 font-bold rounded h-8 w-8 dark:bg-gray-600 dark:text-gray-200"
      >#</span
    >
    <span>
      <slot />
    </span></a
  >
</h2>
