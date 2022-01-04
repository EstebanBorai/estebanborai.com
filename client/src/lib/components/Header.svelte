<script lang="ts">
	import { onMount } from 'svelte';

	let useDarkMode = false;

	onMount(() => {
		useDarkMode =
			localStorage.theme === 'dark' ||
			(!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches);

		if (useDarkMode) {
			document.documentElement.classList.add('dark');
			localStorage.theme = 'dark';
		} else {
			document.documentElement.classList.remove('dark');
			localStorage.theme = 'light';
		}
	});

	function toggleDarkMode(): void {
		if (useDarkMode) {
			document.documentElement.classList.remove('dark');
			localStorage.theme = 'light';
			useDarkMode = false;
		} else {
			document.documentElement.classList.add('dark');
			localStorage.theme = 'dark';
			useDarkMode = true;
		}
	}
</script>

<header class="flex justify-center px-4 h-[70px] bg-light-background dark:bg-dark-background">
	<div class="safe-zone flex items-center justify-between">
		<h1>Esteban Borai</h1>
		<nav>
			<ul class="flex">
				<li class="mx-2 font-semibold">
					<a href="/"> Home </a>
				</li>
				<li class="mx-2 font-semibold">
					<a href="/about"> About </a>
				</li>
				<li class="mx-2 font-semibold">
					<a href="/notes"> Notes </a>
				</li>
				<li>
					<button on:click={toggleDarkMode}>
						{#if useDarkMode}
							<span>🌞</span>
						{:else}
							<span>🌔</span>
						{/if}
					</button>
				</li>
			</ul>
		</nav>
	</div>
</header>
