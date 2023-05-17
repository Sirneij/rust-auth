import type { SeriesAndArticles } from '$lib/utils/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const fetchSeries = async (): Promise<Array<SeriesAndArticles>> => {
		const res = await fetch(`/api/series`);

		return res.ok && (await res.json());
	};

	return {
		series: fetchSeries()
	};
};
