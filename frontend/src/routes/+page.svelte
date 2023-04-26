<script lang="ts">
	import ActixWebSvelte from '$lib/svgs/rust.jpg';
	import { topics } from '$lib/utils/constant';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import type { Topic } from '$lib/utils/types';
	import { flip } from 'svelte/animate';

	let sometopics = [topics[0]],
		next = 1;

	const addTopic = () => {
		if (next < topics.length) {
			sometopics.push(topics[next]);
			sometopics = sometopics;
			next += 1;
		}
	};

	function removeTopic(topic: Topic) {
		sometopics = sometopics.filter((t) => t !== topic);
		next -= 1;
	}
</script>

<svelte:head>
	<title>Written articles | Actix Web & SvelteKit</title>
</svelte:head>

<div
	class="container flex flex-col px-6 py-10 mx-auto space-y-6 lg:h-[32rem] lg:py-16 lg:flex-row lg:items-center"
>
	<div class="w-full lg:w-1/2">
		<div class="lg:max-w-lg">
			<h1 class="text-3xl font-bold tracking-wide text-white lg:text-5xl">
				Authentication system using Actix Web and Sveltekit
			</h1>

			<h2 class="text-lg text-slate-400 mt-3">Articles in this series</h2>

			<div class="mt-8 space-y-5">
				{#each sometopics as topic (topic.id)}
					<p
						in:receive={{ key: topic.id }}
						out:send={{ key: topic.id }}
						class="flex items-center -mx-2 text-gray-200"
						animate:flip
					>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							class="w-6 h-6 mx-2 text-blue-500"
							fill="none"
							viewBox="0 0 24 24"
							stroke="currentColor"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								stroke-width="2"
								d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
							/>
						</svg>

						<a href={topic.url} class="mx-2 text-slate-300 hover:underline hover:text-sky-400"
							>{topic.title}</a
						>

						<button
							on:click={() => removeTopic(topic)}
							type="button"
							class="text-rose-200 rounded px-2 bg-rose-800">-</button
						>
					</p>
				{/each}
				<div class="relative">
					{#if next < topics.length}
						<button
							on:click={addTopic}
							type="button"
							class="absolute right-0 px-2 text-emerald-200 rounded bg-emerald-800"
						>
							+
						</button>
					{/if}
				</div>
			</div>
		</div>
	</div>

	<div class="flex items-center justify-center w-full h-96 lg:w-1/2">
		<img
			class="object-cover w-full h-full mx-auto rounded-lg lg:max-w-2xl"
			src={ActixWebSvelte}
			alt="Rust (actix-web) and Sveltekit"
		/>
	</div>
</div>
