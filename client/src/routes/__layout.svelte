<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';

	import '$lib/styles/global.css';
</script>

<svelte:head>
	<title>Esteban Borai</title>
</svelte:head>

<div class="bg-light-background-alt dark:bg-dark-background-alt">
	<Header />
	<main class="container safe-zone m-auto">
		<slot />
	</main>
	<Footer />
</div>

<style>
	.container {
		min-height: calc(100vh - 70px);
	}
</style>
