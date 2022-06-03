import type { MaybePromise, RequestEvent, ResolveOptions } from '@sveltejs/kit/types/internal';

interface HandleType {
	event: RequestEvent<Record<string, string>>;
	resolve: (
		event: RequestEvent<Record<string, string>>,
		opts?: ResolveOptions | undefined
	) => MaybePromise<Response>;
}

// paths to not apply ssr
const noSSR = ['/demo', '/scan/maptransform', '/debug'];

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }: HandleType) {
	const response = await resolve(event, {
		ssr:!noSSR.some((path) => event.url.pathname.startsWith(path)), // Check if the current route is not in the noSSR array
		transformPage: ({ html }) => html.replace('old', 'new')
	});

	return response;
}
