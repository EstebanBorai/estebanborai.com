<script lang="ts">
  import { goto } from '$app/navigation';
  import { humanDate } from '$lib/utils/date';

  const LANGUAGE_COLOR: {
    [lang: string]: string;
  } = {
    python: '#3572A5',
    rust: '#dea584',
    svelte: '#ff3e00',
    typescript: '#2b7489',
  };

  export let title: string;
  export let description: string;
  export let publishDate: Date;
  export let tags: string[];
  export let slug: string;
  export let previewImageUrl: string;

  let formattedDate = humanDate(publishDate);

  async function handleClick(): Promise<void> {
    await goto(`/notes/${slug}`);
  }
</script>

<li class="mb-4 md:mb-0 last-of-type:mb-0">
  <figure
    class="rounded overflow-hidden pb-4 flex justify-center items-center overflow-hidden h-[150px]"
  >
    <img alt={title} src={previewImageUrl} />
  </figure>
  <header>
    <h3
      class="text-xl py-4 font-extrabold cursor-pointer hover:text-link hover:underline"
      role="link"
      on:click={handleClick}
    >
      {title}
    </h3>
  </header>
  <main class="pb-4">
    <p>{description}</p>
  </main>
  <footer class="flex flex-col">
    <div class="flex mb-2">
      <span class="flex items-center mr-2">
        <figure class="mr-2">
          <!-- <Calendar size={16} /> -->
        </figure>
        <time class="text-sm mr-2" datetime={publishDate.toString()}
          >{formattedDate}</time
        >
      </span>
    </div>
    <ul class="flex flex-wrap">
      {#each tags as tag}
        <li
          class="text-sm mr-2 mb-2 bg-light-background dark:bg-dark-background rounded py-1 px-2"
        >
          {#if tag.toLowerCase() in LANGUAGE_COLOR}
            <span
              class="inline-block mr-1 rounded-full h-2 w-2"
              style="background-color: {LANGUAGE_COLOR[tag.toLowerCase()]};"
            />
          {/if}
          {tag}
        </li>
      {/each}
    </ul>
  </footer>
</li>
