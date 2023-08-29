import cv, { type MatVector, type Mat } from 'opencv-ts';

type Curves = number[];
export type ContourTree = { [key: number]: Curves[] };
export type ContourTreeObject = {
    curves: number[][];
    hierarchy: number[];
};

function zip<K, V>(arr1: K[], arr2: V[]) {
    if (arr1.length != arr2.length) {
        throw 'Arrays have different lengths';
    }

    return arr1.map((k, i) => [k, arr2[i]] as [K, V]);
}

/**
 * Detects the curves in an image, converts internal OpenCV data structure to arrays usable by JavaScript, and removes double contours
 * @param canvas - The canvas to extract the level-curves from. !! For better results try to prepocess the image before passing it to this function !!
 * @returns contours and hierarchy of the image
 */
export async function detectCurvesFromCanvas(canvas: HTMLCanvasElement): Promise<{
    cvContours: MatVector;
    cvHierarchy: Mat;
}> {
    const cvContours = new cv.MatVector(); // this will be used to hold the contours
    const cvHierarchy = new cv.Mat(); // this will be used to hold the hierarchy of the contours

    const image = cv.imread(canvas); // load the image from the canvas

    const gray = new cv.Mat(); // create empty image for holding the grayscale image
    cv.cvtColor(image, gray, cv.COLOR_RGBA2GRAY, 0); // convert image to grayscale

    const blurred = new cv.Mat(); // create image to hold blurred image
    const kernelSize = new cv.Size(5, 5); // set the size of the blur
    cv.GaussianBlur(gray, blurred, kernelSize, 0, 0, cv.BORDER_DEFAULT); // blur the image

    const sharpened = new cv.Mat(); // create new image to hold sharpened image
    cv.addWeighted(gray, 1.5, blurred, -0.5, 0, sharpened); //create a sharpened image by subtracting the original image

    const thresholded = new cv.Mat(); // create empty image for holding the thresholded image
    cv.threshold(sharpened, thresholded, 127, 255, cv.THRESH_BINARY | cv.THRESH_OTSU); // binarize the image by thresholding it


    cv.findContours(thresholded, cvContours, cvHierarchy, cv.RETR_TREE, cv.CHAIN_APPROX_NONE); // get contours out of the image

    // delete Mats to prevent memory leaks
    gray.delete();
    blurred.delete();
    sharpened.delete();
    thresholded.delete();
    image.delete();

    return { cvContours, cvHierarchy };
}

export function matCVToContourHierarchy(contours: MatVector, hierarchy: Mat): ContourTree {
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

    // Validate the contours and hierarchy
    if (contours_array.length == 0 || hierarchy_array.length == 0) {
        throw 'No contours found';
    } else if (contours_array.length != hierarchy_array.length) {
        throw 'The hierarchy array and the contours array have different lengths. Try to rescan the image';
    }

    contours.delete();
    hierarchy.delete();

    const contourTree: ContourTree = {}

    zip(hierarchy_array, contours_array).reduce((obj, [h, k]) => {
        (obj[h] = obj[h] || []).push(k);
        return obj;
    }, contourTree);

    return contourTree;
}


function filterTree(contoursHierarchy: ContourTree) {
    // 1. Remove all contours that are not part of the tree (hierarchy value -1)
    delete contoursHierarchy["-1"];

    // 2. Remove all contours that are too short (less than 3 points )
    // 3. Remove all contours that have an odd number of points (OpenCV returns a flat array of x,y coordinates, so every contour should have an even number of points)
    Object.keys(contoursHierarchy).forEach((key) => {
        const keyNum = parseInt(key);

        if (key in contoursHierarchy && keyNum >= 0) {
            contoursHierarchy[keyNum] = contoursHierarchy[keyNum].filter((contour) => contour.length >= 6 && contour.length % 2 == 0);
        }
    })

    return contoursHierarchy;
}

/**
 * Remove every odd-leveled node from the contours tree
 *
 * @param contours JavaScript (not OpenCV!) array of contours
 * @param hierarchy List of parent nodes for every node
 * @returns *Scientifically* de-duplicated version of the tree
 */
function removeDoubleContours(contoursHierarchy: ContourTree) {
    const level = Object.keys(contoursHierarchy).map((key) => parseInt(key));

    const evenLevels = level.filter((key) => key % 2 == 0)
    const oddLevels = level.filter((key) => key % 2 == 1)

    // remove the odd levels if there are more even levels
    // otherwise remove the even levels
    if (evenLevels.length > oddLevels.length) {
        oddLevels.forEach((key) => {
            delete contoursHierarchy[key];
        })
    } else {
        evenLevels.forEach((key) => {
            delete contoursHierarchy[key];
        })
    }
}

/**
 * Detects the curves in an image, converts internal OpenCV data structure to arrays usable by JavaScript, and removes double contours
 *
 * @param img OpenCV image object
 */
export function getCurvesFromContourHierarchy(contoursHierarchy: ContourTree): ContourTreeObject {

    filterTree(contoursHierarchy); // Filter the tree to remove invalid contours

    // remove the double contours caused by detecting both the inside and the outside of each line
    removeDoubleContours(contoursHierarchy);

    const curves = Object.values(contoursHierarchy).flat();
    const hierarchy = Object.keys(contoursHierarchy).map((key) => parseInt(key));

    return { curves, hierarchy };
}