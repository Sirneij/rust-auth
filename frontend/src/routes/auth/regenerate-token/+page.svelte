<script lang="ts">
	import { page } from '$app/stores';
	import { notification } from '$lib/stores/notification.store';
	import { sadEmoji } from '$lib/utils/constant';

	let reason = '';

	if ($page.url.search) {
		reason = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}

	if (reason) {
		$notification = {
			message: `${reason} ${sadEmoji}...`,
			colorName: 'rose'
		};
	}
</script>

<svelte:head>
	<title>Auth - Regenerate Token | Actix Web & SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<form
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center align-middle bg-slate-800 py-4"
	>
		<h1 class="text-center text-2xl font-bold text-sky-400 mb-6">Regenerate token</h1>

		<div class="w-3/4 mb-2">
			<input
				type="email"
				name="email"
				id="email"
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Email address"
				required
			/>
		</div>

		<div class="w-3/4 mt-4">
			<button
				type="submit"
				class="py-2 bg-sky-800 w-full rounded text-blue-50 font-bold hover:bg-sky-700"
			>
				Regenerate
			</button>
		</div>
	</form>
</div>
