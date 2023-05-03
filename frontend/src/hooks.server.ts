import { BASE_API_URI } from '$lib/utils/constant';
import type { User } from '$lib/utils/types';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	if (event.locals.user) {
		// if there is already a user  in session load page as normal
		return await resolve(event);
	}
	// get cookies from browser
	const session = event.cookies.get('id');

	if (!session) {
		// if there is no session load page as normal
		return await resolve(event);
	}

	// find the user based on the session
	const res = await event.fetch(`${BASE_API_URI}/users/current-user/`, {
		credentials: 'include',
		headers: {
			Cookie: `sessionid=${session}`
		}
	});

	if (!res.ok) {
		// if there is no session load page as normal
		return await resolve(event);
	}

	// if `user` exists set `events.local`
	const response: User = (await res.json()) as User;

	event.locals.user = response;

	// load page as normal
	return await resolve(event);
};
