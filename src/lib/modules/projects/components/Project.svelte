<script lang="ts" context="module">
  export enum Language {
    Rust,
    TypeScript,
  }
</script>

<script lang="ts">
  export let name: string;
  export let description: string;
  export let image: string;
  export let language: Language | null = null;
  export let repository: string | null = null;
  export let website: string | null = null;
</script>

<li
  class="bg-light-background dark:bg-dark-background p-4 rounded flex flex-col items-center mb-4 md:mb-0 last-of-type:mb-0 mx-auto w-full md:w-[290px] md:h-[380px]"
>
  <h3 class="text-lg mx-auto text-center my-2">{name}</h3>
  {#if image}
    <figure class="my-4">
      <img src={image} alt="Project Name" height="140" width="140" />
    </figure>
  {/if}
  <p class="w-full text-center ellipsis h-22 mb-2">{description}</p>
  <div class="flex my-2 text-sm justify-evenly items-center w-full self-end">
    {#if repository}
      <a target="_blank" href={repository}>Repository</a>
    {/if}
    {#if website}
      <a target="_blank" href={website}>Website</a>
    {/if}
    {#if language}
      <a target="_blank">Rust</a>
    {/if}
  </div>
</li>
