<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { apiResponse } from '$lib/stores/api.response.store';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import { BASE_API_URI, happyEmoji } from '$lib/utils/constant';
	import {
		isValidEmail,
		isValidPasswordMedium,
		isValidPasswordStrong
	} from '$lib/utils/helpers/input.validation';
	import { post } from '$lib/utils/requests/posts.requests';
	import type { ApiResponse, CustomError } from '$lib/utils/types';
	import { flip } from 'svelte/animate';
	import { scale } from 'svelte/transition';

	let email = '',
		password = '',
		first_name = '',
		last_name = '',
		confirmPassword = '',
		errors: Array<CustomError> = [],
		fieldsError = { email: '', password: '', first_name: '', last_name: '', confirmPassword: '' },
		isValid = false;

	const handleRegister = async () => {
		isValid = true;
		if (!isValidEmail(email)) {
			isValid = false;
			fieldsError.email = 'That email address is invalid.';
		} else {
			fieldsError.email = '';
		}
		if (!isValidPasswordMedium(password)) {
			isValid = false;
			fieldsError.password =
				'Password is not valid. Password must contain six characters or more and has at least one lowercase and one uppercase alphabetical character or has at least one lowercase and one numeric character or has at least one uppercase and one numeric character.';
		} else {
			fieldsError.password = '';
		}
		if (confirmPassword.trim() !== password.trim()) {
			isValid = false;
			fieldsError.confirmPassword = 'Password does not match.';
		} else {
			fieldsError.confirmPassword = '';
		}

		if (isValid) {
			loading.setLoading(true, 'Please wait while we register you...');
			const [res, err] = await post($page.data.fetch, `${BASE_API_URI}/users/register/`, {
				first_name: first_name,
				last_name: last_name,
				email: email,
				password: password
			});
			if (err.length > 0) {
				loading.setLoading(false);
				errors = err;
			} else {
				loading.setLoading(false);
				const response: ApiResponse = res;
				$notification = {
					message: `You have successfully registered ${happyEmoji}...`,
					colorName: 'green'
				};
				$apiResponse = {
					message: response.message ? response.message : '',
					status: response.status ? response.status : ''
				};
				await goto('/auth/confirming');
			}
		}
	};
</script>

<svelte:head>
	<title>Auth - Register | Actix Web & SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<form
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center bg-slate-800 py-4"
		on:submit|preventDefault={handleRegister}
	>
		<h1 class="text-center text-2xl font-bold text-sky-400 mb-6">Register</h1>

		{#if errors}
			{#each errors as error (error.id)}
				<p
					class="text-center text-rose-600"
					transition:scale|local={{ start: 0.7 }}
					animate:flip={{ duration: 200 }}
				>
					{error.error}
				</p>
			{/each}
		{/if}

		<div class="w-3/4 mb-2">
			<input
				type="email"
				name="email"
				id="email"
				bind:value={email}
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Email address"
				required
			/>
			{#if fieldsError.email}
				<span class="text-center text-rose-600" transition:scale|local={{ start: 0.7 }}>
					{fieldsError.email}
				</span>
			{/if}
		</div>

		<div class="w-3/4 mb-2">
			<input
				type="text"
				name="first_name"
				bind:value={first_name}
				id="first-name"
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="First name"
				required
			/>
		</div>
		<div class="w-3/4 mb-2">
			<input
				type="text"
				name="last_name"
				id="last-name"
				bind:value={last_name}
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Last name"
				required
			/>
		</div>

		<div class="w-3/4 mb-2">
			<input
				type="password"
				name="password"
				bind:value={password}
				id="password"
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Password"
				required
			/>
			{#if fieldsError.password}
				<span class="text-center text-rose-600" transition:scale|local={{ start: 0.7 }}>
					{fieldsError.password}
				</span>
			{/if}
		</div>

		<div class="w-3/4 mb-6">
			<input
				type="password"
				name="confirm-password"
				bind:value={confirmPassword}
				id="confirm-password"
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Confirm password"
				required
			/>
			{#if fieldsError.confirmPassword}
				<span class="text-center text-sm text-rose-600" transition:scale|local={{ start: 0.7 }}>
					{fieldsError.confirmPassword}
				</span>
			{/if}
		</div>

		<div class="w-3/4">
			<button
				type="submit"
				class="py-2 bg-sky-800 w-full rounded text-blue-50 font-bold hover:bg-sky-700"
			>
				Create account
			</button>
		</div>
		<div class="w-3/4 flex flex-row justify-center mt-1">
			<span class="text-sm text-sky-400">
				Already have an account?<a
					href="/auth/login"
					class="ml-2 text-slate-400 underline hover:text-sky-400"
				>
					Login.
				</a>
			</span>
		</div>
	</form>
</div>
