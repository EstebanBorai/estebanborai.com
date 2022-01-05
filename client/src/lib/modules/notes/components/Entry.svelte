<script lang="ts">
	import Calendar from '$lib/components/icons/Calendar.svelte';
	import Hourglass from '$lib/components/icons/Hourglass.svelte';

	export let title: string;
	export let description: string;
	export let publishDate: Date;
	export let estimatedReadTime: number;
	export let tags: string[];
</script>

<ul>
	<li class="md:px-0 pb-8 px-4">
		<header>
			<h3 class="text-2xl font-extrabold">{title}</h3>
		</header>
		<main class="py-4">
			<p>{description}</p>
		</main>
		<footer class="flex flex-col">
			<div class="flex mb-2">
				<span class="flex items-center mr-2">
					<Calendar className="h-4 w-4 mr-2" />
					<time class="text-sm mr-2">{publishDate}</time>
				</span>
				<span class="flex items-center">
					<Hourglass className="h-4 w-4 mr-2" />
					<span>
						{estimatedReadTime} min read
					</span>
				</span>
			</div>
			<ul class="flex">
				{#each tags as tag}
					<li class="text-sm mr-2 bg-light-background dark:bg-dark-background rounded py-1 px-2">
						{tag}
					</li>
				{/each}
			</ul>
		</footer>
	</li>
</ul>
