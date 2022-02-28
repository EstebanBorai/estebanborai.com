<script lang="ts">
	import GitHub from '$lib/components/icons/GitHub.svelte';
	import LinkedIn from '$lib/components/icons/LinkedIn.svelte';
	import StackOverflow from '$lib/components/icons/StackOverflow.svelte';
	import Twitter from '$lib/components/icons/Twitter.svelte';
</script>

<section
	class="container flex flex-col justify-center items-center md:flex-row md:mx-auto md:justify-evenly md:items-center"
>
	<div class="flex md:items-center md:justify-center w-1/2 py-4 w-[300px]">
		<figure class="w-[300px]">
			<img
				class="rounded-full"
				src="https://avatars.githubusercontent.com/u/34756077?v=4"
				height="300"
				width="300"
				alt="A selfie"
			/>
		</figure>
	</div>
	<article class="flex flex-col py-4 md:w-3/6 w-4/6">
		<strong class="text-3xl md:text-4xl py-4 text-center md:text-left">Hi I'm Esteban Borai,</strong
		>
		<p class="text-center md:text-left">
			Software and Computer Science enthusiast. Using Rust 🦀 and TypeScript 👨🏻‍💻.
		</p>
		<ul class="flex py-4 mx-auto md:mx-0">
			<li class="mr-4">
				<a
					rel="noopener noreferrer"
					href="https://stackoverflow.com/users/9888500/esteban-borai"
					target="_blank"
				>
					<StackOverflow className="w-6 h-6" />
				</a>
			</li>
			<li class="mr-4">
				<a rel="noopener noreferrer" href="https://github.com/EstebanBorai" target="_blank">
					<GitHub className="w-6 h-6 fill-current" />
				</a>
			</li>
			<li class="mr-4">
				<a
					rel="noopener noreferrer"
					href="https://www.linkedin.com/in/esteban-b-241ba0135/"
					target="_blank"
				>
					<LinkedIn className="w-6 h-6" />
				</a>
			</li>
			<li class="mr-4">
				<a rel="noopener noreferrer" href="https://twitter.com/EstebanBorai" target="_blank">
					<Twitter className="w-6 h-6" />
				</a>
			</li>
		</ul>
	</article>
</section>

<style>
	.container {
		min-height: calc(100vh - 70px);
	}
</style>
