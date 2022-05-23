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

// this function is used for debugging, it draws the curves detected by the detectCurves function
export function drawCurves(image) {
    const [contours, hierarchy] = detectCurves(image) // get contours and hierarchy from the detectContours function
    contourColor = new cv.Scalar(255, 255, 0, 255) // this is the color yellow, which will be used to draw the contours

    cv.drawContours(image, contours, -1, contourColor, 1, cv.LINE_8, hierarchy, 100) // draw the contours in the image
    return image
}