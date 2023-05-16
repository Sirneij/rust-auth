<script lang="ts">
	import { page } from '$app/stores';
	import { happyEmoji } from '$lib/utils/constant';
	let message = '';
	if ($page.url.search) {
		message = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}
</script>

<svelte:head>
	<title>Auth - Email Sent | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<div
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center divide-y bg-slate-800 py-4"
	>
		<h1 class="text-center text-2xl font-bold text-emerald-400 mb-2">Email sent!</h1>

		<p class="text-emerald-600 text-justify px-4 py-4">
			{message}
			{happyEmoji}...
		</p>
	</div>
</div>
