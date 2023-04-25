import { writable } from 'svelte/store';

export const notification = writable({
	message: '',
	borderColor: '',
	textTopColor: '',
	textBottomColor: ''
});
