import type { MaybePromise, RequestEvent, ResolveOptions } from '@sveltejs/kit/types/internal';

interface HandleType {
	event: RequestEvent<Record<string, string>>;
	resolve: (
		event: RequestEvent<Record<string, string>>,
		opts?: ResolveOptions | undefined
	) => MaybePromise<Response>;
}

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }: HandleType) {
	const response = await resolve(event, {
		ssr: !event.url.pathname.startsWith('/demo'),
		transformPage: ({ html }) => html.replace('old', 'new')
	});

	return response;
}
