<script lang="ts">
	import { applyAction, enhance, type SubmitFunction } from '$app/forms';
	import { page } from '$app/stores';
	import ImageInput from '$lib/component/Input/ImageInput.svelte';
	import Modal from '$lib/component/Modal/Modal.svelte';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import Avatar from '$lib/svgs/teamavatar.png';
	import { happyEmoji } from '$lib/utils/constant';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import type { ActionData } from './$types';

	export let form: ActionData;

	let showModal = false;

	const open = () => (showModal = true);
	const close = () => (showModal = false);

	const handleUpdate: SubmitFunction = async () => {
		loading.setLoading(true, 'Please wait while your profile is being updated...');

		return async ({ result }) => {
			loading.setLoading(false);

			if (result.type === 'success' || result.type === 'redirect') {
				$notification = {
					message: `Your profile has been saved successfully ${happyEmoji}...`,
					colorName: 'emerald'
				};
				close();
			}
			await applyAction(result);
		};
	};
</script>

<svelte:head>
	{#if $page.data.user}
		<title>
			Auth - About {`${$page.data.user.first_name} ${$page.data.user.last_name}`} | Auth Systems with
			SvelteKit
		</title>
	{/if}
</svelte:head>

{#if $page.data.user}
	<h2 style="text-align:center">
		{`${$page.data.user.first_name} ${$page.data.user.last_name}`} Profile
	</h2>
	<div class="card">
		<img
			src={$page.data.user.thumbnail ? $page.data.user.thumbnail : Avatar}
			alt={`${$page.data.user.first_name} ${$page.data.user.last_name}`}
			style="width:90%; margin:auto;"
		/>
		<h1>{`${$page.data.user.first_name} ${$page.data.user.last_name}`}</h1>

		<div class="details">
			{#if $page.data.user.profile.phone_number}
				<p>
					<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 512 512">
						<path
							d="M164.9 24.6c-7.7-18.6-28-28.5-47.4-23.2l-88 24C12.1 30.2 0 46 0 64C0 311.4 200.6 512 448 512c18 0 33.8-12.1 38.6-29.5l24-88c5.3-19.4-4.6-39.7-23.2-47.4l-96-40c-16.3-6.8-35.2-2.1-46.3 11.6L304.7 368C234.3 334.7 177.3 277.7 144 207.3L193.3 167c13.7-11.2 18.4-30 11.6-46.3l-40-96z"
						/>
					</svg> <span>{$page.data.user.profile.phone_number}</span>
				</p>
			{/if}

			{#if $page.data.user.profile.birth_date}
				<p>
					<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 448 512">
						<path
							d="M96 32V64H48C21.5 64 0 85.5 0 112v48H448V112c0-26.5-21.5-48-48-48H352V32c0-17.7-14.3-32-32-32s-32 14.3-32 32V64H160V32c0-17.7-14.3-32-32-32S96 14.3 96 32zM448 192H0V464c0 26.5 21.5 48 48 48H400c26.5 0 48-21.5 48-48V192z"
						/>
					</svg> <span>{$page.data.user.profile.birth_date}</span>
				</p>
			{/if}

			{#if $page.data.user.profile.github_link}
				<p>
					<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 496 512">
						<path
							d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"
						/>
					</svg>
					<a href={$page.data.user.profile.github_link}>Github</a>
				</p>
			{/if}
		</div>

		<button on:click={open} class="btn">Edit</button>
	</div>
{/if}

{#if showModal}
	<Modal on:close={close}>
		<form
			method="POST"
			action="?/updateUser"
			enctype="multipart/form-data"
			use:enhance={handleUpdate}
		>
			<h2 style="text-align:center">User Profile Update</h2>

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

			<ImageInput avatar={$page.data.user.thumbnail} />

			<input
				type="text"
				name="first_name"
				value={$page.data.user.first_name}
				placeholder="Your first name..."
			/>
			<input
				type="text"
				name="last_name"
				value={$page.data.user.last_name}
				placeholder="Your last name..."
			/>
			<input
				type="tel"
				name="phone_number"
				value={$page.data.user.profile.phone_number ? $page.data.user.profile.phone_number : ''}
				placeholder="Your phone number e.g +2348135703593..."
			/>
			<input
				type="date"
				name="birth_date"
				value={$page.data.user.profile.birth_date ? $page.data.user.profile.birth_date : ''}
				placeholder="Your date of birth..."
			/>
			<input
				type="tel"
				name="github_link"
				value={$page.data.user.profile.github_link ? $page.data.user.profile.github_link : ''}
				placeholder="Your github link e.g https://github.com/Sirneij/..."
			/>
			<button type="submit" class="btn">Update</button>
		</form>
	</Modal>
{/if}
