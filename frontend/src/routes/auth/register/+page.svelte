<script lang="ts">
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import { happyEmoji } from '$lib/utils/constant';
	import { scale } from 'svelte/transition';
	import type { ActionData, SubmitFunction } from './$types';
	import { applyAction, enhance } from '$app/forms';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';

	export let form: ActionData;

	const handleRegister: SubmitFunction = async () => {
		loading.setLoading(true, 'Please wait while we register you...');

		return async ({ result }) => {
			loading.setLoading(false);
			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `You have successfully registered ${happyEmoji}...`,
					colorName: `emerald`
				};
			}
			await applyAction(result);
		};
	};
</script>

<svelte:head>
	<title>Auth - Register | Auth Systems with SvelteKit</title>
</svelte:head>

<form class="form" action="?/register" method="POST" use:enhance={handleRegister}>
	<h1 style="text-align:center">Register</h1>

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

	<input type="email" name="email" id="email" placeholder="Email address" required />
	{#if form?.fieldsError && form?.fieldsError.email}
		<span class="text-center text-rose-600 text-xs" transition:scale|local={{ start: 0.7 }}>
			{form?.fieldsError.email}
		</span>
	{/if}

	<input type="text" name="first_name" id="first-name" placeholder="First name" required />

	<input type="text" name="last_name" id="last-name" placeholder="Last name" required />

	<input type="password" name="password" id="password" placeholder="Password" required />
	{#if form?.fieldsError && form?.fieldsError.password}
		<span class="text-center text-rose-600 text-xs" transition:scale|local={{ start: 0.7 }}>
			{form?.fieldsError.password}
		</span>
	{/if}

	<input
		type="password"
		name="confirm_password"
		id="confirm-password"
		placeholder="Confirm password"
		required
	/>
	{#if form?.fieldsError && form?.fieldsError.confirmPassword}
		<span class="text-center text-xs text-rose-600" transition:scale|local={{ start: 0.7 }}>
			{form?.fieldsError.confirmPassword}
		</span>
	{/if}

	<button type="submit" class="btn"> Create account </button>

	<span class="text-xs text-sky-400" style="display:block; text-align: center; margin-top: 0.5rem">
		Already have an account?<a href="/auth/login" class="ml-2 text-slate-400"> Login. </a>
	</span>
</form>
