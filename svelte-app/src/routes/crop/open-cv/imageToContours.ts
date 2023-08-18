import cv from 'opencv-ts';
import { getCurves } from './detectCurves';
import { contourLines } from '$lib/stores/contourLineStore';

/**
 * Loads the contents of the provided canvas into opencv. Opencv is then used to extract
 * the set of level-curves from this image, after which they are stored in the appropriate store.
 *
 * @param canvas {HTMLCanvasElement} The canvas to extract the level-curves from.
 * @returns void
 */
export function imageToContoursGammaCV(canvas: HTMLCanvasElement) {
    // Transform the canvas content to an OpenCV Mat construct
    const opencv_mat = cv.imread(canvas);

    try {
        // Set contour line store to the detected contour lines with hierarchy
        const { curves, hierarchy } = getCurves(opencv_mat);

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
            size: { width: canvas.width, height: canvas.height }
        });

        cv.imshow(canvas, opencv_mat);
    } catch (message) {
        opencv_mat.delete();

        return message as string;
    }

    opencv_mat.delete();
}