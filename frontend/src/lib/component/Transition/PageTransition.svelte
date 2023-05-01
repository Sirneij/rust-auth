<script lang="ts">
	import { slide } from 'svelte/transition';
	export let key: string;
	export let duration = 300;
</script>

{#key key}
	<div in:slide={{ duration, delay: duration }} out:slide={{ duration }}>
		<slot />
	</div>
{/key}
