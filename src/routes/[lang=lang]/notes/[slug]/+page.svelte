<script>
  export let data;
</script>

<article class="flex justify-center mx-auto p-4 md:p-2 md:w-11/12">
  <svelte:component this={data.content} />
</article>
