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

	// delete Mats to prevent memory leaks
	gray.delete();
	blurred.delete();
	sharpened.delete();
	thresholded.delete();

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

	contours.delete();
	hierarchy.delete();

	// return { curves: contours_array, hierarchy: hierarchy_array };  // For debugging purposes, if you want to check the contours without de-duplication
	
	// remove the double contours caused by detecting both the inside and the outside of each line
	[contours_array, hierarchy_array] = removeDoubleContours(contours_array, hierarchy_array);
	// remove the root, which is a rectangle around the entire image
	[contours_array, hierarchy_array] = removeRootContour(contours_array, hierarchy_array);

	validateContours(contours_array, hierarchy_array); //throw exceptions if something is wrong with the scan

	return {curves: contours_array, hierarchy: hierarchy_array};
	
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
 * @returns *Scientifically* de-duplicated version of the tree
 */
function removeDoubleContours(contours: number[][], hierarchy: number[]): [number[][], number[]] {
	const levels = getLevels(hierarchy);

	let contours_dedup: number[][] = []; 
	let hierarchy_dedup: number[] = [];
	let new_indices: number[] = []; // this array will hold the indices of the contours in the deduplicated array

	

	// For every even-leveled node, add its contour and parent index to the deduplicated arrays
	// Also add the index where it is added to the new_indices array
	// For every odd-leveled node, add the index of its parent in new_indices to the array
	levels.forEach((level, i) => {
		if (level % 2 == 0) {
			new_indices.push(contours_dedup.length);
			contours_dedup.push(contours[i]);
			hierarchy_dedup.push(hierarchy[i]);
		} else {
			new_indices.push(new_indices[hierarchy[i]]);
		}
	});

	// update the hierarchy to correspond to the deduplicated array
	hierarchy_dedup.forEach((parent, i) => {
		if(hierarchy_dedup[i] == -1) {
			hierarchy_dedup[i] = -1; // root node has index -1
		} else {
			hierarchy_dedup[i] = new_indices[hierarchy_dedup[i]];
		}
	})

	return [contours_dedup, hierarchy_dedup];
}

/**
 * Remove the root from the contours tree
 *
 * @param contours JavaScript (not OpenCV!) array of contours
 * @param hierarchy List of parent nodes for every node
 * @returns The tree without the root
 */
function removeRootContour(contours: number[][], hierarchy: number[]): [number[][], number[]] {

	//remove first element from the contours and hierarchy array
	contours.shift();
	hierarchy.shift();

	//subtract 1 from all hierarchy references, this also changes all references of 0 to -1
	hierarchy.forEach((parent, i) => {
		hierarchy[i] = hierarchy[i] - 1;
	})


	return [contours, hierarchy];
}

/**
 * Throws an exception if something is wrong with the level curve tree
 *
 * @param contours JavaScript array of contours
 * @param hierarchy List of parent nodes for every node
 */
 function validateContours(contours: number[][], hierarchy: number[]): void {
	if (contours.length == 0 || hierarchy.length == 0) {
		throw('No contours found');	
	}

	contours.forEach((contour, i) => {
		if(contour.length <= 3) {
			throw("A detected contour was too small, please try again");
		}

		if(hierarchy[i] < -1) { // original scan had more than one root
			console.log(contours, hierarchy)
			throw("Something went wrong while scanning. Try to not include things other than the level curves in your scan, or take a new picture");
		}
	})
}