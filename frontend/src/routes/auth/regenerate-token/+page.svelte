<script lang="ts">
	import { page } from '$app/stores';
	import { notification } from '$lib/stores/notification.store';
	import { happyEmoji, sadEmoji } from '$lib/utils/constant';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { scale } from 'svelte/transition';
	import type { ActionData, SubmitFunction } from './$types';
	import { loading } from '$lib/stores/loading.store';
	import { applyAction, enhance } from '$app/forms';

	export let form: ActionData;

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

	const handleGenerate: SubmitFunction = async () => {
		loading.setLoading(true, 'Please wait while we regenerate your token...');

		return async ({ result }) => {
			loading.setLoading(false);
			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `You have successfully regenerated a new token ${happyEmoji}...`,
					colorName: `emerald`
				};
			}
			await applyAction(result);
		};
	};
</script>

<svelte:head>
	<title>Auth - Regenerate Token | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<form class="form" method="POST" use:enhance={handleGenerate}>
		<h1 style="text-align:center">Regenerate token</h1>
		{#if form?.errors}
			{#each form?.errors as error (error.id)}
				<p
					class="text-center text-rose-600"
					in:receive={{ key: error.id }}
					out:send={{ key: error.id }}
				>
					{error.error}
				</p>
			{/each}
		{/if}
		<input type="email" name="email" id="email" placeholder="Registered e-mail address" required />
		{#if form?.fieldsError && form?.fieldsError.email}
			<span class="text-center text-rose-600 text-xs" transition:scale|local={{ start: 0.7 }}>
				{form?.fieldsError.email}
			</span>
		{/if}

		<button type="submit" class="btn">Regenerate</button>
	</form>
</div>
