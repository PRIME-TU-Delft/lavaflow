/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores if aframe is loaded globaly
 */

 import { writable } from 'svelte/store';

 export const aframeLoaded = writable<boolean>(false);