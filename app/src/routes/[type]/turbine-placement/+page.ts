import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load = (({ params }) => {
	const validTypes = new Set(['3d', 'ar', 'web-xr']);

	if (validTypes.has(params.type)) {
		return params;
	}

	throw error(404, params.type + ' is not a valid type');
}) satisfies PageLoad;
