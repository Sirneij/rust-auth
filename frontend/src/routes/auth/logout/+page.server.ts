import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BASE_API_URI } from '$lib/utils/constant';
import type { CustomError } from '$lib/utils/types';

export const load: PageServerLoad = async ({ locals }) => {
	// redirect user if not logged in
	if (!locals.user) {
		throw redirect(302, `/auth/login?next=/auth/logout`);
	}
};

export const actions: Actions = {
	default: async ({ fetch, cookies }) => {
		const requestInitOptions: RequestInit = {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				Cookie: `sessionid=${cookies.get('id')}`
			}
		};

		const res = await fetch(`${BASE_API_URI}/users/logout/`, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors: Array<CustomError> = [];
			errors.push({ error: response.error, id: 0 });
			return fail(400, { errors: errors });
		}

		// eat the cookie
		cookies.delete('id', { path: '/' });

		// redirect the user
		throw redirect(302, '/auth/login');
	}
};
