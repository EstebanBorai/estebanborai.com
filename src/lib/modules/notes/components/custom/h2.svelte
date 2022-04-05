<script lang="ts">
  let data = null;
  let href = '';
  let id = '';

  $: {
    if (typeof data?.innerText === 'string') {
      id = data?.innerText?.toLowerCase().split(' ').join('-');
      href = `#${id}`;
    }
  }
</script>

<h2 {id} class="hover:underline hover:text-blue-200" bind:this={data}>
  <a class="text-inherit no-underline" {href}>
    <slot />
  </a>
</h2>
