import { browser } from '$app/environment';
import imageStore from '$lib/stores/imageStore';
import sizeStore from '$lib/stores/sizeStore';
import { redirect } from '@sveltejs/kit';
import { get } from 'svelte/store';
import type { PageLoad } from './$types';

export const ssr = false; // Is only required because openCV is a pain in the butt

export const load = (() => {
	if (!browser) return {};

	const sizeStoreValue = get(sizeStore);

	if (!sizeStoreValue.width || !sizeStoreValue.height) {
		const error = encodeURIComponent('It is not possible to select markers without a image size');
		throw redirect(307, `/capture?error=${error}`);
	}

	const imageStoreValue = get(imageStore);
	if (!imageStoreValue) {
		const error = encodeURIComponent('It is not possible to select markers without a image');
		throw redirect(307, `/capture?error=${error}`);
	}

	return {};
}) satisfies PageLoad;
