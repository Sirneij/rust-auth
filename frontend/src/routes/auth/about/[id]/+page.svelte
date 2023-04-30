<script lang="ts">
	import { page } from '$app/stores';
	import ImageInput from '$lib/component/Input/ImageInput.svelte';
	import Modal from '$lib/component/Modal/Modal.svelte';
	import { loading } from '$lib/stores/loading.store';
	import { notification } from '$lib/stores/notification.store';
	import { loggedInUser } from '$lib/stores/user.store';
	import Avatar from '$lib/svgs/teamavatar.png';
	import { BASE_API_URI, happyEmoji } from '$lib/utils/constant';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { post } from '$lib/utils/requests/posts.requests';
	import type { CustomError, User } from '$lib/utils/types';

	let showModal = false,
		errors: Array<CustomError> = [],
		image: string | Blob,
		avatar: string | null = $loggedInUser.thumbnail,
		first_name = $loggedInUser.first_name,
		last_name = $loggedInUser.last_name,
		phone_number = $loggedInUser.profile.phone_number,
		birth_date = $loggedInUser.profile.birth_date,
		github_link = $loggedInUser.profile.github_link;

	const open = () => (showModal = true);
	const close = () => (showModal = false);

	async function handleUpdate(event: Event) {
		loading.setLoading(true, 'Please wait while your profile is being updated...');
		let data = new FormData();
		if (first_name !== $loggedInUser.first_name) {
			data.append('first_name', first_name);
		}
		if (last_name !== $loggedInUser.last_name) {
			data.append('last_name', last_name);
		}
		if (image !== null && image !== undefined) {
			data.append('thumbnail', image);
		}
		if (phone_number && phone_number !== $loggedInUser.profile.phone_number) {
			data.append('phone_number', phone_number);
		}
		if (birth_date && birth_date !== $loggedInUser.profile.birth_date) {
			data.append('birth_date', birth_date);
		}
		if (github_link && github_link !== $loggedInUser.profile.github_link) {
			data.append('github_link', github_link);
		}

		const [res, err] = await post(
			$page.data.fetch,
			`${BASE_API_URI}/users/update-user/`,
			data,
			'include',
			'PATCH'
		);

		if (err.length > 0) {
			loading.setLoading(false);
			errors = err;
		} else {
			loading.setLoading(false);
			(event.target as HTMLFormElement).reset();
			$notification = {
				message: `Your profile has been saved successfully ${happyEmoji}...`,
				colorName: 'green'
			};
			loggedInUser.set(res as User);
			close();
		}
	}
</script>

<svelte:head>
	<script src="https://kit.fontawesome.com/e9a50f7f89.js" crossorigin="anonymous"></script>

	<title>
		Auth - About {`${$loggedInUser.first_name} ${$loggedInUser.last_name}`} | Actix Web & SvelteKit
	</title>
</svelte:head>

<h2 style="text-align:center">
	{`${$loggedInUser.first_name} ${$loggedInUser.last_name}`} Profile
</h2>
<div class="card">
	<img
		src={$loggedInUser.thumbnail ? $loggedInUser.thumbnail : Avatar}
		alt={`${$loggedInUser.first_name} ${$loggedInUser.last_name}`}
		style="width:90%; margin:auto;"
	/>
	<h1>{`${$loggedInUser.first_name} ${$loggedInUser.last_name}`}</h1>

	<div class="details">
		{#if $loggedInUser.profile.phone_number}
			<p><i class="fa-solid fa-phone" /> <span>{$loggedInUser.profile.phone_number}</span></p>
		{/if}

		{#if $loggedInUser.profile.birth_date}
			<p><i class="fa-solid fa-calendar" /> <span>{$loggedInUser.profile.birth_date}</span></p>
		{/if}

		{#if $loggedInUser.profile.github_link}
			<p><i class="fa-brands fa-github" /><a href={$loggedInUser.profile.github_link}>Github</a></p>
		{/if}
	</div>

	<button on:click={open}>Edit</button>
</div>

{#if showModal}
	<Modal on:close={close}>
		<form enctype="multipart/form-data" on:submit|preventDefault={handleUpdate}>
			<h2 style="text-align:center">User Profile Update</h2>

			{#if errors}
				{#each errors as error (error.id)}
					<p
						class="text-center text-rose-600"
						in:receive={{ key: error.id }}
						out:send={{ key: error.id }}
					>
						{error.error}
					</p>
				{/each}
			{/if}

			<ImageInput bind:image title="Upload user image" bind:avatar bind:errors />

			<input
				type="text"
				name="first_name"
				bind:value={first_name}
				placeholder="Your first name..."
			/>
			<input type="text" name="last_name" bind:value={last_name} placeholder="Your last name..." />
			<input
				type="tel"
				name="phone_number"
				bind:value={phone_number}
				placeholder="Your phone number e.g +2348135703593..."
			/>
			<input
				type="date"
				name="birth_date"
				bind:value={birth_date}
				placeholder="Your date of birth..."
			/>
			<input
				type="tel"
				name="github_link"
				bind:value={github_link}
				placeholder="Your github link e.g https://github.com/Sirneij/..."
			/>
			<button type="submit">Update</button>
		</form>
	</Modal>
{/if}

<style>
	:root {
		--tw-bg-opacity: 1;
		--tw-text-opacity: 1;
	}
	h1,
	h2 {
		color: rgb(14 165 233 / var(--tw-text-opacity));
	}
	h2 {
		font-size: 1.5rem;
	}
	h1 {
		font-size: 2rem;
	}

	.card {
		box-shadow: 0px 4px 6px 0px rgba(0, 0, 0, 0.75);
		-webkit-box-shadow: 0px 4px 6px 0px rgba(0, 0, 0, 0.75);
		-moz-box-shadow: 0px 4px 6px 0px rgba(0, 0, 0, 0.75);
		max-width: 20rem;
		margin: auto;
		text-align: center;
	}

	button {
		border: none;
		outline: 0;
		display: inline-block;
		padding: 0.5rem;
		color: rgb(239 246 255 / var(--tw-bg-opacity));
		background-color: rgb(7 89 133 / var(--tw-bg-opacity));
		text-align: center;
		cursor: pointer;
		width: 100%;
		font-size: 18px;
	}

	.details {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: center;
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
		color: rgb(239 246 255 / var(--tw-bg-opacity));
	}

	.details p i {
		opacity: 0.6;
		margin-right: 0.3rem;
	}
	.details p:not(:last-of-type) {
		margin-right: 1rem;
	}
	.details p:not(:last-of-type) {
		border-right: 2px solid rgb(14 165 233 / var(--tw-text-opacity));
	}

	.details p span,
	.details p a {
		margin-right: 0.3rem;
	}

	button:hover,
	a:hover {
		opacity: 0.7;
	}
	a:hover {
		color: rgb(14 165 233 / var(--tw-text-opacity));
		text-decoration: underline;
	}

	form {
		border-radius: 5px;
		background-color: rgb(30 41 59);
		padding: 1.25rem;
	}
	input {
		width: 100%;
		padding: 0.75rem 1.25rem;
		margin: 0.25rem 0;
		display: inline-block;
		border: none;
		outline: none;
		background-color: #0f172a;
		color: rgb(14 165 233);
		border-radius: 4px;
		box-sizing: border-box;
	}
	::-webkit-input-placeholder {
		/* Edge */
		color: rgb(148 163 184);
	}

	:-ms-input-placeholder {
		/* Internet Explorer 10-11 */
		color: rgb(148 163 184);
	}

	::placeholder {
		color: rgb(148 163 184);
	}
</style>
