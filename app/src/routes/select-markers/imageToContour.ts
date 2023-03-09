import { goto } from '$app/navigation';
import type Draggable from '$lib/data/draggable';
import { contourLines } from '$lib/stores/contourLineStore';
import sizeStore from '$lib/stores/sizeStore';
import imageStore from '$lib/stores/imageStore';
import cv from 'opencv-ts';
import { get } from 'svelte/store';
import { getCurves } from './open-cv/detectCurves';
import removePerspective from './open-cv/removePerspective';
import * as gm from 'gammacv'

/**
 * takes the <img id="foregroundImage" /> and converts it to contour lines with hierarchy.
 * @param points Draggable points that are the edges of the sub-image
 * @returns
 */
export default async function imageToCountours(imageURL="", points: [Draggable, Draggable, Draggable, Draggable]) {
	const $sizeStore = get(sizeStore);
	if (!$sizeStore.width || !$sizeStore.height)
		return "No size found for the image. Please go back to the 'Capture' page and try again.";

	const [width, height] = [$sizeStore.width, $sizeStore.height];

	// Transform the image (from imageStore) into a gammacv tensor
	const gammacvInputTensor = await gm.imageTensorFromURL(imageURL, "uint8", [$sizeStore.height*2, $sizeStore.width*2, 4])

	const cannyEdgesOperation = gm.cannyEdges(
		gm.sobelOperator(gm.gaussianBlur(gammacvInputTensor, 3, 1)), 0.25, 0.75
	)

	// Extract the tensor output
	const gammacvOutputTensor: any = gm.tensorFrom(cannyEdgesOperation)

	// Create and initialize the GammaCV session, to acquire GPU acceperation
	const gammacvSession = new gm.Session()
	gammacvSession.init(cannyEdgesOperation)

	// Run the canny-edges operation
	gammacvSession.runOp(cannyEdgesOperation, 0, gammacvOutputTensor);

	// Select the canvas from the DOM
	const hiddenCanvas: any = document.getElementById("foregroundCanvasImage")

	gm.canvasFromTensor(hiddenCanvas, gammacvOutputTensor)

	imageStore.set(hiddenCanvas.toDataURL('image/png'))

	// Grab image from DOM
	const mat = cv.imread('foregroundCanvasImage');

	// Fetch the marker coordinates of the draggable buttons
	const markerCoords: number[] = [];
	for (const p of points) {
		markerCoords.push(p.x - p.offsetX);
		markerCoords.push(p.y - p.offsetY);
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
