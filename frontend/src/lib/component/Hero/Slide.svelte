<script lang="ts">
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import type { SeriesAndArticles } from '$lib/utils/types';

	export let series: SeriesAndArticles;
</script>

<div
	class="hero fade"
	style="background-image: linear-gradient(rgba(0, 0, 0, 0.55), rgba(0, 0, 0, 0.55)), url({series.image})"
>
	<div class="hero-text">
		<h1 class="text-3xl font-bold tracking-wide text-sky lg:text-5xl">
			{series.name}
		</h1>
		<h2 class="text-lg text-slate-400 mt-3">
			Articles in this series ({series.articles.length} part)
		</h2>
		<div class="mt-8 space-y-5">
			{#each series.articles as topic (topic.id)}
				<p
					in:receive={{ key: topic.id }}
					out:send={{ key: topic.id }}
					class="flex items-center -mx-2 text-gray-200"
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

					{#if topic}
						<a href={topic.url} class="mx-2 text-slate-300 hover:underline hover:text-sky-400">
							{topic.title}
						</a>
					{/if}
				</p>
			{:else}
				<p>No articles for this series yet...</p>
			{/each}
		</div>
	</div>
</div>
