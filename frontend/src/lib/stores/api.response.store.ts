import { writable } from 'svelte/store';

export const apiResponse = writable({ message: '', status: '' });
