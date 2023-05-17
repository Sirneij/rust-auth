<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';
	import Slide from '$lib/component/Hero/Slide.svelte';

	export let data: PageData;

	$: ({ series } = data);

	let slideIndex = 1,
		plusSlides: any,
		showSlides: any;

	onMount(() => {
		showSlides = (n: number) => {
			let i;
			let slides = document.getElementsByClassName('hero');
			if (slides) {
				if (n > slides.length) {
					slideIndex = 1;
				}
				if (n < 1) {
					slideIndex = slides.length;
				}
				for (i = 0; i < slides.length; i++) {
					(slides[i] as HTMLElement).style.display = 'none';
				}

				(slides[slideIndex - 1] as HTMLElement).style.display = 'block';
			}
		};

		showSlides(slideIndex);

		// Next/previous controls
		plusSlides = (n: number) => {
			showSlides((slideIndex += n));
		};
	});
</script>

<svelte:head>
	<title>Written articles | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="hero-container">
	{#if series && Array.isArray(series)}
		{#each series as s (s.id)}
			<Slide bind:series={s} />
		{/each}
	{/if}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<span class="prev" on:click={() => plusSlides(-1)}>&#10094;</span>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<span class="next" on:click={() => plusSlides(-1)}>&#10095;</span>
</div>
