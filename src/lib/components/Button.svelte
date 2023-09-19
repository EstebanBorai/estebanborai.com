<script lang="ts">
  export let title: string;
  export let disabled: boolean;
</script>

<button
  class="cursor-pointer inline-block text-center bg-blue-600 rounded-full text-sm py-1 px-4"
  type="button"
  {disabled}
  on:click
>
  <span class="text-white">{title}</span>
</button>
