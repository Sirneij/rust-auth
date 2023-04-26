import { goto } from '$app/navigation';
import { notification } from '$lib/stores/notification.store';
import { isAuthenticated } from '$lib/stores/user.store';
import { BASE_API_URI, happyEmoji, sadEmoji } from '../constant';
import type { ApiResponse } from '../types';
import { post } from './posts.requests';

/**
 * Logs a user out of the application.
 * @file lib/utils/requests/logout.requests.ts
 * @param {typeof fetch} svelteKitFetch - Fetch object from sveltekit
 * @param {string} [redirectUrl='/auth/login'] - URL to redirect to after logout.
 */
export const logout = async (svelteKitFetch: typeof fetch, redirectUrl = '/auth/login') => {
	const [res, err] = await post(
		svelteKitFetch,
		`${BASE_API_URI}/users/logout/`,
		undefined,
		'include'
	);
	if (err.length > 0) {
		notification.set({
			message: `${err[0].error} ${sadEmoji}...`,
			colorName: 'rose'
		});
	} else {
		const response: ApiResponse = res;
		notification.set({
			message: `${response.message} ${happyEmoji}...`,
			colorName: 'green'
		});

		isAuthenticated.set(false);
		if (redirectUrl !== '') {
			await goto(redirectUrl);
		}
	}
};
