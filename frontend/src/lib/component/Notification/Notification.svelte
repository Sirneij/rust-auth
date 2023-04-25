<script lang="ts">
	import { notification } from '$lib/stores/notification.store';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { onDestroy } from 'svelte';

	let visible: boolean,
		ms = 3000,
		timeout: any;
	const onMessageChange = (message: string, ms: number) => {
		clearTimeout(timeout);
		if (!message) {
			// hide Alert if message is empty
			visible = false;
		} else {
			// show alert
			visible = true;
			// and hide it after ms milliseconds
			if (ms > 0) timeout = setTimeout(() => (visible = false), ms);
		}
	};
	// whenever the alert store or the ms props changes run onMessageChange
	$: onMessageChange($notification.message, ms);
	// make sure we clean-up the timeout
	onDestroy(() => clearTimeout(timeout));
</script>

{#if visible}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="absolute left-1/2 -translate-x-1/2 -translate-y-0 top-0 p-1 max-w-2xl flex rounded border {$notification.borderColor}"
		in:receive={{ key: Math.floor(Math.random() * 100) }}
		out:send={{ key: Math.floor(Math.random() * 100) }}
		role="alert"
		on:click={() => (visible = false)}
	>
		<div class="flex flex-col items-start text-sm">
			<p class="{$notification.textBottomColor} font-medium antialiased">
				<strong>
					{#if $notification.textBottomColor.includes('green')}
						Well done!:
					{:else if $notification.textBottomColor.includes('rose')}
						Oh snap!:
					{/if}
				</strong>
				<span class="ml-2">{$notification.message}</span>
			</p>
		</div>
	</div>
{/if}
