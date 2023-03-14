import { goto } from '$app/navigation';
import type Draggable from '$lib/data/draggable';
import { contourLines } from '$lib/stores/contourLineStore';
import sizeStore from '$lib/stores/sizeStore';
import imageStore from '$lib/stores/imageStore';
import cv from 'opencv-ts';
import { get } from 'svelte/store';
import { getCurves } from './open-cv/detectCurves';
import removePerspective, { removePerspectiveGammaCV } from './open-cv/removePerspective';
import * as gm from 'gammacv'

/**
 * takes the <img id="foregroundImage" /> and converts it to contour lines with hierarchy.
 * @param points Draggable points that are the edges of the sub-image
 * @returns
 */
export default function imageToCountours(points: [Draggable, Draggable, Draggable, Draggable]) {
	const $sizeStore = get(sizeStore);
	if (!$sizeStore.width || !$sizeStore.height)
		return "No size found for the image. Please go back to the 'Capture' page and try again.";

	const [width, height] = [$sizeStore.width, $sizeStore.height];

	// Grab image from DOM
	const mat = cv.imread("foregroundImage");

	// Fetch the marker coordinates of the draggable buttons
	const markerCoords: number[] = [];
	for (const p of points) {
		markerCoords.push(p.mappedX);
		markerCoords.push(p.mappedY);
	}

	// Apply the perspective transformation using the selected marker coords
	const result = removePerspective(mat, markerCoords, width, height);

	try {
		// Set contour line store to the detected contour lines with hierarchy
		const { curves, hierarchy } = getCurves(result);

		// Convert the OpenCV Mat to a array of tuples for mountain model construction
		const contourTuples: [number, number][][] = curves.map((contour) => {
			const contourTuple: [number, number][] = [];

			for (let i = 0; i < contour.length - 1; i += 2) {
				contourTuple.push([contour[i], contour[i + 1]]);
			}

			return contourTuple;
		});

		contourLines.setup({
			curves: contourTuples,
			hierarchy: hierarchy,
			size: { width, height }
		});

		cv.imshow('canvasOutput', result);

		goto('/preview');
	} catch (message) {
		result.delete();
		mat.delete();

		return message as string;
	}

	result.delete();
	mat.delete();
}



export async function extractSelectedArea(points: [Draggable, Draggable, Draggable, Draggable], canvas: HTMLCanvasElement) {
	
	const $sizeStore = get(sizeStore);
	if (!$sizeStore.width || !$sizeStore.height)
		return "No size found for the image. Please go back to the 'Capture' page and try again.";

	const [width, height] = [$sizeStore.width, $sizeStore.height];

	// Grab image from DOM
	//const mat = cv.imread("foregroundImage");

	// Fetch the marker coordinates of the draggable buttons
	const markerCoords: number[] = [];
	for (const p of points) {
		markerCoords.push(p.mappedX);
		markerCoords.push(p.mappedY);
	}

	// Apply the perspective transformation using the selected marker coords

	const sourceTensor = await gm.imageTensorFromURL(get(imageStore), "uint8", [height, width, 4])

	const result = removePerspectiveGammaCV(
		sourceTensor,
		markerCoords,
		width,
		height
	)

	//mat.delete()

	console.log(result)

	const context = canvas.getContext('2d')
	gm.canvasFromTensor(canvas, sourceTensor)


}