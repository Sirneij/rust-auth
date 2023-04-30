import { notification } from '$lib/stores/notification.store';
import { isAuthenticated } from '$lib/stores/user.store';
import { angryEmoji } from '$lib/utils/constant';
import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageLoad = async ({ params }) => {
	if (!get(isAuthenticated)) {
		notification.set({
			message: `You are not logged in ${angryEmoji}...`,
			colorName: `red`
		});
		throw redirect(302, `/auth/login?next=/auth/about/${params.id}`);
	}
};
