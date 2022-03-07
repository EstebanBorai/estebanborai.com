<script lang="ts">
  import { goto } from '$app/navigation';

  import Calendar from '$lib/components/icons/Calendar.svelte';
  import { humanDate } from '$lib/utils/date';

  export let title: string;
  export let description: string;
  export let publishDate: Date;
  export let tags: string[];
  export let slug: string;

  let formattedDate = humanDate(publishDate);

  async function handleClick(): Promise<void> {
    await goto(`/notes/${slug}`);
  }
</script>

<li class="md:px-0 pb-8 px-4">
  <header>
    <h3
      class="text-2xl font-extrabold cursor-pointer hover:text-link"
      role="link"
      on:click={handleClick}
    >
      {title}
    </h3>
  </header>
  <main class="py-4">
    <p>{description}</p>
  </main>
  <footer class="flex flex-col">
    <div class="flex mb-2">
      <span class="flex items-center mr-2">
        <Calendar className="h-4 w-4 mr-2" />
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
          {tag}
        </li>
      {/each}
    </ul>
  </footer>
</li>
