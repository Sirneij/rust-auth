<script lang="ts">
	import { applyAction, enhance, type SubmitFunction } from '$app/forms';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import { happyEmoji } from '$lib/utils/constant';
	import type { ActionData } from './$types';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { page } from '$app/stores';

	export let form: ActionData;

	const handleLogin: SubmitFunction = async () => {
		loading.setLoading(true, 'Please wait while we log you in...');

		return async ({ result }) => {
			loading.setLoading(false);
			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `Login successfull ${happyEmoji}...`,
					colorName: `emerald`
				};
			}
			await applyAction(result);
		};
	};

	let message = '';

	if ($page.url.search) {
		message = $page.url.search.split('=')[1].replaceAll('%20', ' ');
	}

	if (message) {
		$notification = {
			message: `${message} ${happyEmoji}...`,
			colorName: 'emerald'
		};
	}
</script>

<svelte:head>
	<title>Auth - Login | Auth Systems with SvelteKit</title>
</svelte:head>

<form class="form" method="POST" action="?/login" use:enhance={handleLogin}>
	<h1 style="text-align:center">Login</h1>

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

	<input type="hidden" name="next" value={$page.url.searchParams.get('next')} />

	<input type="email" name="email" id="email" placeholder="Email address" required />

	<input type="password" name="password" id="password" placeholder="Password" required />

	<p style="text-align: right; margin-bottom: 0.5rem">
		<a href="/auth/password/request-change" class="text-sm text-slate-400"> Forgot password? </a>
	</p>

	<button type="submit" class="btn"> Login </button>

	<p class="text-sm text-sky-400" style="text-align: center; margin-top: 0.5rem">
		No account?
		<a href="/auth/register" class="ml-2 text-slate-400">Create an account.</a>
	</p>
</form>
