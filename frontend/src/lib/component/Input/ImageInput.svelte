<script lang="ts">
	export let avatar: string | null;
	let thumbnail: HTMLInputElement;
	const onFileSelected = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target && target.files) {
			let reader = new FileReader();
			reader.readAsDataURL(target.files[0]);
			reader.onload = (e) => {
				avatar = e.target?.result as string;
			};
		}
	};
</script>

<div id="app">
	<h3>Upload user image</h3>

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
	<svg
		xmlns="http://www.w3.org/2000/svg"
		on:click={() => {
			thumbnail.click();
		}}
		class="upload"
		viewBox="0 0 512 512"
	>
		<path
			d="M220.6 121.2L271.1 96 448 96v96H333.2c-21.9-15.1-48.5-24-77.2-24s-55.2 8.9-77.2 24H64V128H192c9.9 0 19.7-2.3 28.6-6.8zM0 128V416c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H271.1c-9.9 0-19.7 2.3-28.6 6.8L192 64H160V48c0-8.8-7.2-16-16-16H80c-8.8 0-16 7.2-16 16l0 16C28.7 64 0 92.7 0 128zM168 304a88 88 0 1 1 176 0 88 88 0 1 1 -176 0z"
		/>
	</svg>

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
		height: 2.7rem;
		width: 2.7rem;
		cursor: pointer;
		fill: rgb(14 165 233);
	}
	.avatar {
		display: flex;
		height: 8rem;
		width: 8rem;
	}
</style>
