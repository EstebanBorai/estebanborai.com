<script lang="ts">
  import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';

	import '$lib/styles/global.css';
</script>

<svelte:head>
  <title>Esteban Borai</title>
</svelte:head>

<Header />
<main>
  <slot />
</main>
<Footer />