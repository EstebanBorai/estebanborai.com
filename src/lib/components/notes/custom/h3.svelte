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

<h3 {id} class="text-xl font-display relative group py-2" bind:this={data}>
  <a class="no-underline" {href} on:click={focusElement}>
    <span
      class="invisible top-[10px] group-hover:visible absolute left-[-2rem] flex items-center justify-center bg-gray-200 font-semibold rounded h-6 w-6 dark:bg-gray-600 dark:text-gray-200"
      >#</span
    >
    <span>
      <slot />
    </span></a
  >
</h3>
