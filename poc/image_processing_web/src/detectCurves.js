

// this function binarizes the input, then uses cv.findContours to find edges in the image
function detectCurves(image) {
    let gray = new cv.Mat()
    cv.cvtColor(image, gray, cv.COLOR_RGBA2GRAY, 0) // convert image to grayscale

    let thresholded = new cv.Mat()
    cv.threshold(gray, thresholded, 127, 255, cv.THRESH_BINARY) // binarize the image

    let contours = new cv.MatVector()
    let hierarchy = new cv.Mat()
    cv.findContours(thresholded, contours, hierarchy, cv.RETR_TREE, cv.CHAIN_APPROX_SIMPLE) // get contours out of the image

    return [contours, hierarchy]
}

// this function is used for debugging, it draws the curves detected by the detectCurves function
function drawCurves(image) {
    const [contours, hierarchy] = detectCurves(image)
    contourColor = new cv.Scalar(255, 255, 0, 255)
    cv.drawContours(image, contours, -1, contourColor, 1, cv.LINE_8, hierarchy, 100)
    return image
}