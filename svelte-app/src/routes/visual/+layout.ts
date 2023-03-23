import { browser } from '$app/environment';
import { gltfStore } from '$lib/stores/gltfStore';
import { redirect } from '@sveltejs/kit';
import { get } from 'svelte/store';
import type { LayoutLoad } from './$types';

export const load = (() => {
	if (!browser) return {};

	const gltfStoreValue = get(gltfStore);

	if (!gltfStoreValue?.gltf_url) {
		const error = encodeURIComponent('Gltf file is not loaded');
		throw redirect(307, `/preview?error=${error}`);
	}

	return {};
}) satisfies LayoutLoad;
