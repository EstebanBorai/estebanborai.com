<script lang="ts">
  export let data;
</script>

<div id="note-container">
  <svelte:component this={data.content} />
</div>
