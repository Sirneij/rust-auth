import { BASE_SERIES_API_URI } from '$lib/utils/constant';
import { json, type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch }) => {
	const res = await fetch(`${BASE_SERIES_API_URI}/users/series`);

	const response = res.ok && (await res.json());

	return json(response, { status: 200 });
};
