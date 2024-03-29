/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores the image data for the image component
 */

import { writable } from 'svelte/store';

export default writable<string>();
