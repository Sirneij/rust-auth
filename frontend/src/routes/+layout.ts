import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent, data, fetch, url }) => {
	await parent();
	const { user } = data;
	return { fetch, url: url.pathname, user };
};
