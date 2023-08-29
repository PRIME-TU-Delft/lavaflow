import { contourLines } from '$lib/stores/contourLineStore';
import { matCVToContourHierarchy, getCurvesFromContourHierarchy, detectCurvesFromCanvas } from './detectCurves';

/**
 * Draws the provided curves on the provided canvas.
 * @param canvas Debug canvas
 * @param curves Curves to draw
 */
function debugContours(canvas: HTMLCanvasElement, curves: number[][]) {
    const ctx = canvas.getContext('2d');

    if (!ctx) throw 'Could not get canvas context';

    ctx.strokeStyle = 'red';
    ctx.lineWidth = 5;

    for (const curve of curves) {
        ctx.beginPath();
        ctx.moveTo(curve[0], curve[1]);
        for (let i = 2; i < curve.length; i += 2) {
            let x = curve[i];
            let y = curve[i + 1];

            ctx.lineTo(x, y);
        }
        ctx.stroke();

    }
}

/**
 * Loads the contents of the provided canvas into opencv. Opencv is then used to extract
 * the set of level-curves from this image, after which they are stored in the appropriate store.
 *
 * @param canvas {HTMLCanvasElement} The canvas to extract the level-curves from.
 * @param debug {boolean} Whether or not to draw the level-curves on the canvas.
 * @returns void
 */
export async function imageToContoursGammaCV(canvas: HTMLCanvasElement, debug = false) {
    try {
        const { cvContours, cvHierarchy } = await detectCurvesFromCanvas(canvas);
        const contourTree = matCVToContourHierarchy(cvContours, cvHierarchy);
        const { curves, hierarchy } = getCurvesFromContourHierarchy(contourTree);

        const contourTuples: [number, number][][] = [];

        if (debug) {
            debugContours(canvas, curves);
        }

        for (const curve of curves) {
            const contourTuple: [number, number][] = [];

            for (let i = 0; i < curve.length - 1; i += 2) {
                contourTuple.push([curve[i], curve[i + 1]]);
            }

            contourTuples.push(contourTuple);
        }

        contourLines.setup({
            curves: contourTuples,
            hierarchy: hierarchy,
            size: { width: canvas.width, height: canvas.height }
        });
    } catch (message) {

        return message as string;
    }
}