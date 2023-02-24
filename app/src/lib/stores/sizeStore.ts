/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores size of the image, video, or canvas
 */

import { writable } from 'svelte/store';

export default writable<{ width: number; height: number }>();
