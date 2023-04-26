import type { User } from '$lib/utils/types';
import { writable, type Writable } from 'svelte/store';

export const isAuthenticated = writable(false);

export const loggedInUser: Writable<User> = writable();
