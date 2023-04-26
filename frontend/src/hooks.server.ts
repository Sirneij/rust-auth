import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	// if (event.url.pathname.startsWith('/auth/change/password')) {
	// console.log(event.cookies.getAll());
	// }

	return await resolve(event);
};
