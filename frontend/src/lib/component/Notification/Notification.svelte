<script lang="ts">
	import { notification } from '$lib/stores/notification.store';
	import { receive, send } from '$lib/utils/helpers/animate.crossfade';
	import { onDestroy } from 'svelte';

	let visible: boolean,
		ms = 5000,
		timeout: any,
		textColor = 'text-emerald-600',
		bgColor = 'border-emerald-300 bg-emerald-100',
		text = 'Well done!:';
	const onMessageChange = (message: string, ms: number) => {
		clearTimeout(timeout);
		if (!message) {
			// hide Alert if message is empty
			visible = false;
		} else {
			// show alert
			visible = true;
			// and hide it after ms milliseconds
			if (ms > 0)
				timeout = setTimeout(() => {
					visible = false;
					notification.set({ message: '', colorName: '' });
				}, ms);
		}
	};
	// whenever the alert store or the ms props changes run onMessageChange
	$: onMessageChange($notification.message, ms);
	// make sure we clean-up the timeout
	onDestroy(() => clearTimeout(timeout));

	$: {
		if ($notification.colorName === 'rose') {
			bgColor = 'border-rose-300 bg-rose-100';
			textColor = 'text-rose-600';
			text = 'Oh snap!:';
		} else if ($notification.colorName === 'blue') {
			bgColor = 'border-blue-300 bg-blue-100';
			textColor = 'text-blue-600';
			text = 'Heads up!:';
		} else if ($notification.colorName === 'orange') {
			bgColor = 'border-orange-300 bg-orange-100';
			textColor = 'text-orange-600';
			text = 'Warning!:';
		}
	}
</script>

{#if visible}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		class="absolute left-1/2 -translate-x-1/2 -translate-y-0 top-0 p-1 max-w-2xl flex rounded border {bgColor}"
		in:receive={{ key: Math.floor(Math.random() * 100) }}
		out:send={{ key: Math.floor(Math.random() * 100) }}
		role="alert"
		on:click={() => (visible = false)}
	>
		<div class="flex flex-col items-start text-sm">
			<p class="{textColor} font-medium antialiased">
				<strong>
					{text}
				</strong>
				<span class="ml-2">{$notification.message}</span>
			</p>
		</div>
	</div>
{/if}
