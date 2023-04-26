import type { Loading } from '$lib/utils/types';
import { writable, type Writable } from 'svelte/store';

const newLoading = () => {
	const { subscribe, update, set }: Writable<Loading> = writable({
		status: 'IDLE',
		message: ''
	});

	const setNavigate = (isNavigating: boolean) => {
		update(() => {
			return {
				status: isNavigating ? 'NAVIGATING' : 'IDLE',
				message: ''
			};
		});
	};

	const setLoading = (isLoading: boolean, message = '') => {
		update(() => {
			return {
				status: isLoading ? 'LOADING' : 'IDLE',
				message: isLoading ? message : ''
			};
		});
	};

	return { subscribe, update, set, setNavigate, setLoading };
};

export const loading = newLoading();
