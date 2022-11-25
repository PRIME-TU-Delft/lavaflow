import type { Load } from '@sveltejs/kit';

// Check if browser suports webxr (AR)
export const load: Load = async () => {
	const webXRSupport =
		((await navigator?.xr?.isSessionSupported('immersive-ar')) as boolean) || false;

	return {
		webXRSupport
	};
};
