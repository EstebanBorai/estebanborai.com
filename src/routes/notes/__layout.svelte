<script lang="ts">
	import '$lib/styles/note.css';
	import '$lib/styles/prism-one-dark.css';

	export let metadata;
</script>

{metadata}

<div class="safe-zone note-container">
	<slot />
</div>

<style>
	@media only screen and (max-width: 920px) {
		.note-container {
			padding: 0 1rem;
		}
	}
</style>
