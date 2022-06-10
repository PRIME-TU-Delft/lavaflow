/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores if the site is in debug mode
 */

import { writable } from 'svelte/store';

export const debugMode = writable<boolean>(false);
