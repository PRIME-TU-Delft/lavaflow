import { browser } from '$app/environment';
import sizeStore from '$lib/stores/sizeStore';
import { get } from 'svelte/store';
import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load = (() => {
	if (browser) {
		const size = get(sizeStore);

		// TODO add check for contour lines

		if (!size.width || !size.height) {
			const error = encodeURIComponent('Preview is not available without a size');

			throw redirect(307, `/capture?error=${error}`);
		}
	}

	return {};
}) satisfies PageLoad;
