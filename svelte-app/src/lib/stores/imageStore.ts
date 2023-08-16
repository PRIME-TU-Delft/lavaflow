/**
 * Svelte stores, store reactive global data for more info:
 * https://svelte.dev/docs#run-time-svelte-store
 *
 * This stores the image data for the image component
 */

import { writable } from 'svelte/store';


// This store has place for the image that the user capture, as well as the four corner-coordinates
// that were calculated by the edge-detection algorithm. This store is filled in 'capture' and its
// data is used in all subsequent pages.
type CapturedImage = {
    img: string,
    corners: {
        topLeft: [number, number],
        topRight: [number, number],
        bottomLeft: [number, number],
        bottomRight: [number, number]
    }
}

export default writable<CapturedImage>();
