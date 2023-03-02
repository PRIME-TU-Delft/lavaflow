import type LavaError from './LavaError';

/**
 * Monad for handling errors
 * @param url - URL object
 * @param handle - function to handle error
 */
export function getError(url: URL, handle: (err: string) => LavaError): { error?: LavaError } {
	if (url.searchParams.has('error')) {
		const error = url.searchParams.get('error') as string;

		return { error: handle(decodeURIComponent(error)) };
	}

	return {};
}
