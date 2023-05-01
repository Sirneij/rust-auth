<script lang="ts">
	import { applyAction, enhance, type SubmitFunction } from '$app/forms';
	import { page } from '$app/stores';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import JohnImage from '$lib/svgs/john.svg';
	import Avatar from '$lib/svgs/teamavatar.png';
	import { happyEmoji } from '$lib/utils/constant';

	const handleLogout: SubmitFunction = () => {
		loading.setLoading(true, 'Please wait while we log you out...');
		return async ({ result }) => {
			loading.setLoading(false);
			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `Logout successfull ${happyEmoji}...`,
					colorName: `emerald`
				};
			}
			await applyAction(result);
		};
	};
</script>

<header aria-label="Page Header" class="mb-6">
	<nav class="mx-auto max-w-screen-xl px-4 py-4 sm:px-6 lg:px-8">
		<div class="flex items-center justify-end gap-4">
			<div class="flex items-center gap-4">
				<a
					href="https://github.com/Sirneij"
					class="block shrink-0 rounded-full bg-white p-2.5 text-gray-600 shadow-sm hover:text-gray-700"
				>
					<span class="sr-only">Programmer</span>
					<img src={JohnImage} alt="John Idogun" class="h-6 w-6 rounded-full object-cover" />
				</a>
			</div>

			<span aria-hidden="true" class="block h-6 w-px rounded-full bg-gray-200" />

			<a
				href="/"
				class="block shrink-0 {$page.url.pathname === `/`
					? 'text-sky-500'
					: 'text-white'} hover:text-sky-400"
			>
				/
			</a>
			{#if !$page.data.user}
				<a
					href="/auth/login"
					class="block shrink-0 {$page.url.pathname === `/auth/login`
						? 'text-sky-500'
						: 'text-white'} hover:text-sky-400"
				>
					Login
				</a>
			{:else}
				<a href="/auth/about/{$page.data.user.id}" class="block shrink-0">
					<span class="sr-only">{$page.data.user.first_name} Profile</span>
					<img
						alt={$page.data.user.first_name}
						src={$page.data.user.thumbnail ? $page.data.user.thumbnail : Avatar}
						class="h-10 w-10 rounded-full object-cover"
					/>
				</a>

				<form action="/auth/logout" method="POST" use:enhance={handleLogout}>
					<button type="submit" class="text-white hover:text-sky-400 logout">Logout</button>
				</form>
			{/if}
		</div>
	</nav>
</header>

<style>
	form {
		border-radius: 0;
		background-color: transparent;
		padding: 0;
	}
</style>
