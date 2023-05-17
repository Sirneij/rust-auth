<script>
	import { happyEmoji } from '$lib/utils/constant';
</script>

<svelte:head>
	<title>Auth - Account activated | Auth Systems with SvelteKit</title>
</svelte:head>
<div class="flex items-center justify-center h-[60vh]">
	<div
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center divide-y bg-slate-800 py-4"
	>
		<h1 class="text-center text-2xl font-bold text-sky-400 mb-2">Email address confirmed</h1>

		<p class="text-sky-400 text-justify px-4 py-4">
			Your account has been activated successfully {happyEmoji}!!! You can now
			<a href="/auth/login" class="ml-2 text-slate-400 underline hover:text-sky-400"> Login. </a>
		</p>
	</div>
</div>
