<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { quintOut } from 'svelte/easing';
	import type { TransitionConfig } from 'svelte/transition';

	type ModalParams = { duration?: number };
	type Modal = (node: Element, params?: ModalParams) => TransitionConfig;

	const modal: Modal = (node, { duration = 300 } = {}) => {
		const transform = getComputedStyle(node).transform;

		return {
			duration,
			easing: quintOut,
			css: (t, u) => {
				return `transform:
            ${transform}
            scale(${t})
            translateY(${u * -100}%)
          `;
			}
		};
	};

	const dispatch = createEventDispatcher();
	function closeModal() {
		dispatch('close', {});
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="modal-background" on:click={closeModal} />

<div transition:modal={{ duration: 1000 }} class="modal" role="dialog" aria-modal="true">
	<button title="Close" class="modal-close" on:click={closeModal}>
		<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 384 512">
			<path
				d="M342.6 150.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192 210.7 86.6 105.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L146.7 256 41.4 361.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 301.3 297.4 406.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.3 256 342.6 150.6z"
			/>
		</svg>
	</button>
	<slot />
</div>

<style>
	.modal-background {
		width: 100%;
		height: 100%;
		position: fixed;
		top: 0;
		left: 0;
		background: rgba(0, 0, 0, 0.25);
	}

	.modal {
		position: absolute;
		left: 50%;
		top: 50%;
		max-width: 32rem;
		max-height: calc(100vh - 4rem);
		overflow: auto;
		background: rgb(15, 23, 42);
		box-shadow: 0 0 10px hsl(0 0% 0% / 10%);
		transform: translate(-50%, -50%);
		border-radius: 0.5rem;
	}
	@media (max-width: 769px) {
		.modal {
			width: 90%;
		}
	}
	.modal-close {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
	}
	.modal-close svg {
		fill: rgb(14 165 233 /1);
	}
	.modal-close:hover svg {
		fill: rgb(225 29 72);
	}
</style>
