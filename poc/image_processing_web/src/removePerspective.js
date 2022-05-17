import cv from "opencv-ts";

// this image removes the perspective from an image
// points is an array of eight elements, which are the x and y coordinates of four points
// these points are, in order: the top left corner, the top right corner, the bottom right corner and the bottom left corner
// in short, the points are in clockwise order
export function removePerspective(image, points) {
    // the height and width of the output image
    let outputHeight = 550
    let outputWidth = 1000

    // construct mats (required input for cv.getPerspectiveTransform) with four columns and two rows to represent the four 2d points
    let sourcePoints = cv.matFromArray(4, 2, cv.CV_32FC1, points);
    let outputPoints = cv.matFromArray(4, 2, cv.CV_32FC1, [0, 0, outputWidth, 0, outputWidth, outputHeight, 0, outputHeight]);

    // create the transformation matrix
    let M = cv.getPerspectiveTransform(sourcePoints, outputPoints);

    // create an empty mat for the output image and specify its size
    let result = new cv.Mat();
    let dsize = new cv.Size(outputWidth, outputHeight);

    // use the matrix to apply the transformation
    cv.warpPerspective(image, result, M, dsize, cv.INTER_LINEAR, cv.BORDER_CONSTANT, new cv.Scalar());

    return result
}