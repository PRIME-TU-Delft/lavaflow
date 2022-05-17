import cv from "opencv-ts";


// this function binarizes the input, then uses cv.findContours to find edges in the image
export function detectCurves(image) {
    let gray = new cv.Mat() // create empty image for holding the grayscale image
    cv.cvtColor(image, gray, cv.COLOR_RGBA2GRAY, 0) // convert image to grayscale

    let thresholded = new cv.Mat() // create empty image for holding the thresholded image
    cv.threshold(gray, thresholded, 127, 255, cv.THRESH_BINARY) // binarize the image by thresholding it

    let contours = new cv.MatVector() // this will be used to hold the contours
    let hierarchy = new cv.Mat() // this will be used to hold the hierarchy of the contours
    cv.findContours(thresholded, contours, hierarchy, cv.RETR_TREE, cv.CHAIN_APPROX_SIMPLE) // get contours out of the image

    return [contours, hierarchy]
}


/**
 * Detects the curves in an image, converts internal OpenCV data structure to arrays usable by JavaScript, and removes double contours
 * @param img OpenCV image object
 */
export function getCurves(img) {
    const [contours, hierarchy] = detectCurves(img);  // get contours and hierarchy from the detectContours function

    let contours_array = [];
    let hierarchy_array = [];

    // Loop through contours (OpenCV 2D array), and convert it to a JavaScript array
    for (let i = 0; i < contours.size(); i++) {
        contours_array.push(contours.get(i).data32S);
    }

    // Get every 4th element of the hierarchy array (4th element is the parent node)
    for (let i = 3; i < hierarchy.data32S.length; i += 4) {
        hierarchy_array.push(hierarchy.data32S[i]);
    }

    return [contours_array, hierarchy_array];
}


function getLevels(hierarchy_array) {
    let levels = [];
    for (let parent of hierarchy_array) {
        if (parent == -1) {
            levels.push(0);
        }
        else {
            levels.push(levels[parent] + 1);
        }
    }
    return levels;
}


// this function is used for debugging, it draws the curves detected by the detectCurves function
export function drawCurves(image) {
    const [contours, hierarchy] = detectCurves(image);  // get contours and hierarchy from the detectContours function

    let contourColor = new cv.Scalar(255, 0, 0, 255);  // this is the color yellow, which will be used to draw the contours

    cv.drawContours(image, contours, -1, contourColor, 1, cv.LINE_8, hierarchy, 100) // draw the contours in the image
    return image
}
