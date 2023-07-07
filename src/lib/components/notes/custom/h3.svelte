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

<h3 {id} class="text-xl font-body relative group py-4" bind:this={data}>
  <a class="no-underline" {href} on:click={focusElement}>
    <span>
      <slot />
    </span>
    <span
      class="invisible inline p-1 group-hover:visible bg-gray-200 font-bold rounded h-8 w-8 dark:bg-gray-600 dark:text-gray-200"
    >
      🪴
    </span>
  </a>
</h3>
