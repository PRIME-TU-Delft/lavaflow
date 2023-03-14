import type { Tensor } from 'gammacv';
import cv, { type Mat } from 'opencv-ts';
import * as gm from 'gammacv'

export default function removePerspective(
	image: Mat,
	points: number[],
	width = 800,
	height = 667
): Mat {
	// construct mats (required input for cv.getPerspectiveTransform) with four columns and two rows to represent the four 2d points
	const sourcePoints = cv.matFromArray(4, 2, cv.CV_32FC1, points);
	const outputSize = [0, 0, width, 0, width, height, 0, height];
	const outputPoints = cv.matFromArray(4, 2, cv.CV_32FC1, outputSize);

	console.log(outputPoints)

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

	return result;
}


export function removePerspectiveGammaCV(
	sourceTensor: Tensor,
	points: number[],
	width: number,
	height: number
) {

	const targetTensor = new gm.Tensor('float32', [3, 1, 4]);
	gm.generateTransformMatrix(
		new gm.Rect([10, 10, 100, 15, 100, 150, 15, 150]), // Rect on original image to be projected
		[height, width], // Output dimensions
		targetTensor, // Tensor to be filled
	);
	gm.perspectiveProjection(sourceTensor, targetTensor, [height, width, 4]);

	return targetTensor

}
