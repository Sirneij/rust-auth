<script lang="ts">
	import { page } from '$app/stores';
	import { sadEmoji } from '$lib/utils/constant';

	let reason = '';
	if ($page.url.search) {
		reason = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}
</script>

<svelte:head>
	<title>Auth - Error | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<div
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center divide-y bg-slate-800 py-4"
	>
		<h1 class="text-center text-2xl font-bold text-rose-600 mb-2">Error encountered</h1>

		<p class="text-rose-900 text-justify px-4 py-4">
			{#if reason}
				{reason}
			{:else}
				An error has encountered {sadEmoji}. The prime suspect of this sort of error is an expired
				or used token. We are afraid that you may have to restart the whole process again. We are
				really sorry {sadEmoji}.
			{/if}
		</p>
	</div>
</div>
