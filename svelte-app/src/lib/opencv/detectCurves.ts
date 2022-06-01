import cv, { type MatVector, type Mat } from 'opencv-ts';

export type ContourTree = [number[][], number[]];
export type ContourTreeObject = {
	curves: number[][];
	hierarchy: number[];
};

// this function binarizes the input, then uses cv.findContours to find edges in the image
export function detectCurves(image: Mat): [MatVector, Mat] {
	let gray = new cv.Mat(); // create empty image for holding the grayscale image
	cv.cvtColor(image, gray, cv.COLOR_RGBA2GRAY, 0); // convert image to grayscale

	let blurred = new cv.Mat(); // create image to hold blurred image
	const kernelSize = new cv.Size(5, 5); // set the size of the blur
	cv.GaussianBlur(gray, blurred, kernelSize, 0, 0, cv.BORDER_DEFAULT); // blur the image

	let sharpened = new cv.Mat(); // create new image to hold sharpened image
	cv.addWeighted(gray, 1.5, blurred, -0.5, 0, sharpened); //create a sharpened image by subtracting the original image

	let thresholded = new cv.Mat(); // create empty image for holding the thresholded image
	cv.threshold(sharpened, thresholded, 127, 255, cv.THRESH_BINARY | cv.THRESH_OTSU); // binarize the image by thresholding it

	let contours = new cv.MatVector(); // this will be used to hold the contours
	let hierarchy = new cv.Mat(); // this will be used to hold the hierarchy of the contours
	cv.findContours(thresholded, contours, hierarchy, cv.RETR_TREE, cv.CHAIN_APPROX_NONE); // get contours out of the image

	return [contours, hierarchy];
}

/**
 * Detects the curves in an image, converts internal OpenCV data structure to arrays usable by JavaScript, and removes double contours
 *
 * @param img OpenCV image object
 */
export function getCurves(img: Mat): ContourTreeObject {
	const [contours, hierarchy] = detectCurves(img); // get contours and hierarchy from the detectContours function

	let contours_array: number[][] = [];
	let hierarchy_array: number[] = [];

	// Loop through contours (OpenCV 2D array), and convert it to a JavaScript array
	for (let i = 0; i < contours.size(); i++) {
		contours_array.push(Array.from(contours.get(i).data32S));
	}

	// Get every 4th element of the hierarchy array (4th element is the parent node)
	for (let i = 3; i < hierarchy.data32S.length; i += 4) {
		hierarchy_array.push(hierarchy.data32S[i]);
	}

	// return { curves: contours_array, hierarchy: hierarchy_array };  // For debugging purposes, if you want to check the contours without de-duplication
	return removeDoubleContours(contours_array, hierarchy_array);
}

/**
 * Calculates the depth level of every node in an OpenCV tree
 *
 * @param hierarchy_array array of parents for every node
 * @returns array of depth levels of every node
 */
function getLevels(hierarchy_array: number[]): number[] {
	let levels: number[] = [];
	for (let parent of hierarchy_array) {
		if (parent == -1) {
			levels.push(0);
		} else {
			levels.push(levels[parent] + 1);
		}
	}
	return levels;
}

/**
 * Remove every odd-leveled node from the contours tree
 *
 * @param contours JavaScript (not OpenCV!) array of contours
 * @param hierarchy List of parent nodes for every node
 * @returns Magically de-duplicated version of the tree
 */
function removeDoubleContours(contours: number[][], hierarchy: number[]): ContourTreeObject {
	let levels = getLevels(hierarchy);

	let contours_dedup: number[][] = [];
	let hierarchy_dedup: number[] = [];
	let parent_of_parents: number[] = Array(hierarchy.length);

	// For every odd-leveled node: keep track of their parent
	// For every level-leveled node (which is a child of an odd-leveled node), look up its parent's parent, and set that as its own parent
	levels.forEach((level, i) => {
		if (level % 2 == 1) {
			parent_of_parents[i] = hierarchy[i];
		} else {
			contours_dedup.push(contours[i]);
			let new_parent_index = i == 0 ? -1 : Math.floor(parent_of_parents[hierarchy[i]] / 2); // Root node keeps -1 as index
			hierarchy_dedup.push(new_parent_index);
		}
	});

	return { curves: contours_dedup, hierarchy: hierarchy_dedup };
}
