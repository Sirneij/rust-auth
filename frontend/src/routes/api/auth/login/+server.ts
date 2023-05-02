import { BASE_API_URI } from '$lib/utils/constant';
import type { LoginRequestBody, User } from '$lib/utils/types';
import type { RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async (event) => {
	const data = await event.request.formData();

	const email = data.get('email') as string;
	const password = data.get('password') as string;

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

	const res = await event.fetch(apiURL, requestInitOptions);

	if (!res.ok) {
		const response = await res.json();
		// throw error(res.status, response.error);
		return new Response(JSON.stringify({ success: false, error: response.error }), {
			status: res.status,
			headers: {
				'Content-Type': 'application/json'
			}
		});
	}

	for (const header of res.headers) {
		if (header[0] === 'set-cookie') {
			event.cookies.set('id', header[1].split('=')[1].split(';')[0], {
				httpOnly: true,
				sameSite: 'none',
				path: '/',
				secure: true
			});

			break;
		}
	}
	const response: User = (await res.json()) as User;

	event.locals.user = response;

	return new Response(JSON.stringify({ success: false }), {
		status: 200,
		headers: {
			'Content-Type': 'application/json'
		}
	});
};
