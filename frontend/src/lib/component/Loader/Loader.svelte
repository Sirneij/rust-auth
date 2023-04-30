<script>
	import { loading } from '$lib/stores/loading.store';
	import { onMount } from 'svelte';
	let isPageLoaded = false;
	onMount(() => {
		isPageLoaded = true;
	});
	$: if ($loading.status === 'NAVIGATING') {
		setTimeout(() => {
			if ($loading.status === 'NAVIGATING') {
				$loading.status = 'LOADING';
			}
		}, 500);
	}
</script>

{#if $loading.status === 'LOADING'}
	<div class="loader-container">
		<div class="loader" />
		{#if $loading.message}
			<div class="text-container">
				<p>{$loading.message}</p>
			</div>
		{/if}
	</div>
{/if}

{#if !isPageLoaded}
	<div class="loader-start">
		<div class="loader" />
		<div class="text-container">
			<p>Welcome to www.rust-auth.vercel.app...</p>
		</div>
	</div>
{/if}

<style>
	.loader-container {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: hsla(222, 44%, 13%, 1);
		background: linear-gradient(180deg, #011627 3%, hsla(222, 47%, 11%, 1) 41.35%);
		z-index: 99999;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}
	.loader-start {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: hsla(222, 44%, 13%, 1);
		background: linear-gradient(180deg, #011627 3%, hsla(222, 47%, 11%, 1) 41.35%);
		z-index: 99999;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	/* Source: https://www.freecodecamp.org/news/how-to-create-a-css-only-loader/ */
	.loader {
		--b: 20px; /* border thickness */
		--n: 15; /* number of dashes*/
		--g: 7deg; /* gap  between dashes*/
		--c: rgb(3 105 161); /* the color */
		width: 80px; /* size */
		aspect-ratio: 1;
		border-radius: 50%;
		padding: 1px; /* get rid of bad outlines */
		background: conic-gradient(#0000, var(--c)) content-box;
		--_m: /* we use +/-1deg between colors to avoid jagged edges */ repeating-conic-gradient(
				#0000 0deg,
				#000 1deg calc(360deg / var(--n) - var(--g) - 1deg),
				#0000 calc(360deg / var(--n) - var(--g)) calc(360deg / var(--n))
			),
			radial-gradient(farthest-side, #0000 calc(98% - var(--b)), #000 calc(100% - var(--b)));
		-webkit-mask: var(--_m);
		mask: var(--_m);
		-webkit-mask-composite: destination-in;
		mask-composite: intersect;
		animation: load 1s infinite steps(var(--n));
	}
	@keyframes load {
		to {
			transform: rotate(1turn);
		}
	}

	.text-container {
		padding-top: 1rem;
		display: flex;
		justify-content: center;
		color: rgb(3 105 161);
	}
	.text-container p {
		line-height: 1.5;
		font-weight: 700;
		overflow: hidden; /* Ensures the content is not revealed until the animation */
		border-right: 0.15em solid rgb(3 105 161); /* The typwriter cursor */
		white-space: nowrap; /* Keeps the content on a single line */
		margin: 0 auto; /* Gives that scrolling effect as the typing happens */
		animation: typing 3.5s steps(30, end), blink-caret 0.5s step-end infinite;
	}

	/* The typing effect */
	@keyframes typing {
		from {
			width: 0;
		}
		to {
			width: 100%;
		}
	}
	/* The typewriter cursor effect */
	@keyframes blink-caret {
		from,
		to {
			border-color: transparent;
		}
		50% {
			border-color: rgb(3 105 161);
		}
	}
</style>
