import LavaError from '$lib/data/LavaError';
import type { PageLoad } from './$types';

export const load = (({ url }) => {
	const error = url.searchParams.get('error');

	if (error) {
		return { error: new LavaError('Something went wrong', decodeURIComponent(error)) };
	}

	return {};
}) satisfies PageLoad;
