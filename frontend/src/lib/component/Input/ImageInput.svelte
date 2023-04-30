<script lang="ts">
	import { HIGHEST_IMAGE_UPLOAD_SIZE, IMAGE_UPLOAD_SIZE } from '$lib/utils/constant';
	import { returnFileSize } from '$lib/utils/helpers/image.file.size';
	import type { CustomError } from '$lib/utils/types';
	export let title: string;
	export let image: string | Blob;
	export let avatar: string | null;
	export let errors: Array<CustomError>;
	let thumbnail: HTMLInputElement;
	const onFileSelected = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target && target.files) {
			if (target.files[0].size < HIGHEST_IMAGE_UPLOAD_SIZE) {
				errors = [];
				image = target.files[0];
				let reader = new FileReader();
				reader.readAsDataURL(image);
				reader.onload = (e) => {
					avatar = e.target?.result as string;
				};
			} else {
				errors = [
					...errors,
					{
						id: Math.floor(Math.random() * 100),
						error: `Image size ${returnFileSize(
							target.files[0].size
						)} is too large. Please keep it below ${IMAGE_UPLOAD_SIZE}kB.`
					}
				];
			}
		}
	};
</script>

<div id="app">
	<h1>{title}</h1>

	{#if avatar}
		<img class="avatar" src={avatar} alt="d" />
	{:else}
		<img
			class="avatar"
			src="https://cdn4.iconfinder.com/data/icons/small-n-flat/24/user-alt-512.png"
			alt=""
		/>
	{/if}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<i
		class="upload fa-solid fa-3x fa-camera"
		title="Upload image. Max size is 49kB."
		on:click={() => {
			thumbnail.click();
		}}
	/>

	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="chan"
		on:click={() => {
			thumbnail.click();
		}}
	>
		Choose Image
	</div>
	<input
		style="display:none"
		type="file"
		name="thumbnail"
		accept="image/*"
		on:change={(e) => onFileSelected(e)}
		bind:this={thumbnail}
	/>
</div>

<style>
	#app {
		margin-top: 1rem;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-flow: column;
		color: rgb(148 163 184);
	}
	.upload {
		display: flex;
		height: 50px;
		width: 50px;
		cursor: pointer;
	}
	.avatar {
		display: flex;
		height: 200px;
		width: 200px;
	}
</style>
