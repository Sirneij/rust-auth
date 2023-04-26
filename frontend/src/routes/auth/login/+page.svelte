<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import { isAuthenticated, loggedInUser } from '$lib/stores/user.store';
	import { BASE_API_URI, happyEmoji } from '$lib/utils/constant';
	import { post } from '$lib/utils/requests/posts.requests';
	import type { CustomError, User } from '$lib/utils/types';
	import { flip } from 'svelte/animate';
	import { scale } from 'svelte/transition';

	let email = '',
		password = '',
		errors: Array<CustomError> = [];

	const handleLogin = async () => {
		loading.setLoading(true, 'Please wait while we log you in...');
		const [res, err] = await post(
			$page.data.fetch,
			`${BASE_API_URI}/users/login/`,
			{
				email: email,
				password: password
			},
			'include'
		);
		if (err.length > 0) {
			loading.setLoading(false);
			errors = err;
		} else {
			loading.setLoading(false);
			const response: User = res as User;
			$loggedInUser = {
				id: response['id'],
				email: response['email'],
				first_name: response['first_name'],
				last_name: response['last_name'],
				is_staff: response['is_staff'],
				is_superuser: response['is_superuser'],
				thumbnail: response['thumbnail']
			};
			$isAuthenticated = true;

			$notification = {
				message: `Login successfull ${happyEmoji}...`,
				colorName: `green`
			};

			let nextPage = $page.url.search.split('=')[1];
			if ($page.url.hash) {
				nextPage = `${nextPage}${$page.url.hash}`;
			}
			await goto(nextPage || '/', { noScroll: true });
		}
	};
</script>

<svelte:head>
	<title>Auth - Login | Actix Web & SvelteKit</title>
</svelte:head>

<div class="flex items-center justify-center h-[60vh]">
	<form
		class="w-11/12 md:w-2/3 lg:w-1/3 rounded-xl flex flex-col items-center align-middle bg-slate-800 py-4"
		on:submit|preventDefault={handleLogin}
	>
		<h1 class="text-center text-2xl font-bold text-sky-400 mb-6">Login</h1>

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
		</div>

		<div class="w-3/4 mb-6">
			<input
				type="password"
				name="password"
				id="password"
				bind:value={password}
				class="w-full text-sky-500 placeholder:text-slate-600 border-none focus:ring-0 bg-main-color focus:outline-none py-2 px-3 rounded"
				placeholder="Password"
				required
			/>
		</div>

		<div class="w-3/4 flex flex-row justify-between">
			<div class=" flex items-center gap-x-1">
				<input type="checkbox" name="remember" id="" class=" w-4 h-4" />
				<label for="" class="text-sm text-sky-400">Remember me</label>
			</div>
			<div>
				<a href={null} class="text-sm underline text-slate-400 hover:text-sky-400">Forgot?</a>
			</div>
		</div>

		<div class="w-3/4 mt-4">
			<button
				type="submit"
				class="py-2 bg-sky-800 w-full rounded text-blue-50 font-bold hover:bg-sky-700"
			>
				Login
			</button>
		</div>
		<div class="w-3/4 flex flex-row justify-center mt-1">
			<span class="text-sm text-sky-400">
				No account?
				<a href="/auth/register" class="ml-2 text-slate-400 underline hover:text-sky-400">
					Create an account.
				</a>
			</span>
		</div>
	</form>
</div>
