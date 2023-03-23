import { browser } from '$app/environment';
import sizeStore from '$lib/stores/sizeStore';
import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { getError } from '$lib/data/getError';
import LavaError from '$lib/data/LavaError';

export const load = (({ url }) => {
	const error = getError(url, (err) => {
		return new LavaError('Something went wrong', err);
	});

	if (browser) {
		const size = get(sizeStore);

		if (!size.width || !size.height) {
			const error = encodeURIComponent('Preview is not available without a size');

			throw redirect(307, `/capture?error=${error}`);
		}
	}

	return { ...error };
}) satisfies PageLoad;
