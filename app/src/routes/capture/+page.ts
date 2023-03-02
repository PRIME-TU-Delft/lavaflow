import { getError } from '$lib/data/getError';
import LavaError from '$lib/data/LavaError';
import type { PageLoad } from './$types';

export const load = (({ url }) => {
	return getError(url, (err) => {
		return new LavaError('Something went wrong', err);
	});
}) satisfies PageLoad;
