import cv, { type Mat } from 'opencv-ts';

export default function removePerspective(
	image: Mat,
	points: number[],
	width: number = 800,
	height: number = 667
): Mat {
	// construct mats (required input for cv.getPerspectiveTransform) with four columns and two rows to represent the four 2d points
	const sourcePoints = cv.matFromArray(4, 2, cv.CV_32FC1, points);
	const outputSize = [0, 0, width, 0, width, height, 0, height];
	const outputPoints = cv.matFromArray(4, 2, cv.CV_32FC1, outputSize);

	// create the transformation matrix
	const M = cv.getPerspectiveTransform(sourcePoints, outputPoints);

	// create an empty mat for the output image and specify its size
	const result = new cv.Mat();
	const dsize = new cv.Size(width, height);

	// use the matrix to apply the transformation
	cv.warpPerspective(image, result, M, dsize, cv.INTER_LINEAR, cv.BORDER_CONSTANT, new cv.Scalar());

	// Deallocate all Mat instances. Instances of Size and Scalar do not need to be manually deallocated.
	M.delete();
	sourcePoints.delete();
	outputPoints.delete();
	result.delete();

	return result;
}
