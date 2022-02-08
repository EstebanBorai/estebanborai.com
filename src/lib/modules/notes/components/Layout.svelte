<script lang="ts">
	import Calendar from '$lib/components/icons/Calendar.svelte';
	import { humanDate } from '$lib/utils/date';

	import '../styles/note.css';
	import '../styles/prism-one-dark.css';

	export let title;
	export let date;

	let formattedDate = humanDate(new Date(date));
</script>

<div class="safe-zone note-container">
	<header class="py-4">
		<h1 class="text-3xl py-4">{title}</h1>
		<div class="flex mb-2">
			<span class="flex items-center mr-2">
				<Calendar className="h-4 w-4 mr-2" />
				<time class="text-sm mr-2" datetime={date.toString()}>{formattedDate}</time>
			</span>
		</div>
	</header>
	<slot />
</div>

<style>
	@media only screen and (max-width: 920px) {
		.note-container {
			padding: 0 1rem;
		}
	}
</style>
