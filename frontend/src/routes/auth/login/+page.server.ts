import { fail, type Actions, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { CustomError, LoginRequestBody } from '$lib/utils/types';
import { BASE_API_URI } from '$lib/utils/constant';

export const load: PageServerLoad = async ({ locals }) => {
	// redirect user if logged in
	if (locals.user) {
		throw redirect(302, '/');
	}
};

export const actions: Actions = {
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @param cookies - SvelteKit's cookie object
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	login: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();
		const email = String(formData.get('email'));
		const password = String(formData.get('password'));
		const next = String(formData.get('next'));

		const login: LoginRequestBody = {
			email,
			password
		};

		const apiURL = `${BASE_API_URI}/users/login/`;

		const requestInitOptions: RequestInit = {
			method: 'POST',
			credentials: 'include',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(login)
		};

		const res = await fetch(apiURL, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors: Array<CustomError> = [];
			errors.push({ error: response.error, id: 0 });
			return fail(400, { errors: errors });
		}

		if (res.headers.has('Set-Cookie')) {
			const sessionID = Object.fromEntries(res.headers)['set-cookie'].split(';')[0].split('=')[1];
			cookies.set('id', sessionID, {
				httpOnly: true,
				sameSite: 'lax',
				path: '/',
				secure: true
			});
		}

		throw redirect(303, next || '/');
	}
};
