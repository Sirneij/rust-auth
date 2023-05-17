<script lang="ts">
	import { notification } from '$lib/stores/notification.store';
	import { happyEmoji } from '$lib/utils/constant';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { scale } from 'svelte/transition';
	import type { ActionData, SubmitFunction } from './$types';
	import { loading } from '$lib/stores/loading.store';
	import { applyAction, enhance } from '$app/forms';

	export let form: ActionData;

	const handleRequestChange: SubmitFunction = async () => {
		loading.setLoading(true, 'Please wait while we incept your password change process...');

		return async ({ result }) => {
			loading.setLoading(false);
			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `You have successfully requested a password change ${happyEmoji}...`,
					colorName: `emerald`
				};
			}
			await applyAction(result);
		};
	};
</script>

<svelte:head>
	<title>Auth - Request Password Change | Auth Systems with SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<form class="form" method="POST" use:enhance={handleRequestChange}>
		<h1 style="text-align:center">Request Password Change</h1>
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
		<input type="email" name="email" id="email" placeholder="Verified e-mail address" required />
		{#if form?.fieldsError && form?.fieldsError.email}
			<span class="text-center text-rose-600 text-xs" transition:scale|local={{ start: 0.7 }}>
				{form?.fieldsError.email}
			</span>
		{/if}

		<button type="submit" class="btn">Request change</button>
	</form>
</div>
