import type { CapturedImage } from "$lib/stores/imageStore";
import type { Corners } from "../../capture/suggestedCorners";
import * as gm from 'gammacv';
import { preProcessTensor } from "./preProcessCurves";

/**
 * Turns a GammaCV Corners object into a Rect object.
 * @param corners {Corners} Marker points
 * @returns a GammaCV Rect object
 */
function cornersToRectGm(corners: Corners) {
    // Transform the corner marker to a list of points
    // {topLeft: {x: 1, y: 2}, topRight: {x: 3, y: 4}, ...} => [1, 2, 3, 4, ...]
    const points: number[] = []

    points.push(corners.topLeft.x) // ax
    points.push(corners.topLeft.y) // ay

    points.push(corners.topRight.x) // bx
    points.push(corners.topRight.y) // by

    points.push(corners.bottomRight.x) // cx
    points.push(corners.bottomRight.y) // cy

    points.push(corners.bottomLeft.x) // dx
    points.push(corners.bottomLeft.y) // dy

    return new gm.Rect(points);
}

/**
 * removePerspectiveGammaCV is a wrapper around GammaCV's perspectiveProjection function.
 * @param sourceTensor {gm.Tensor} The image to be transformed
 * @param corners {Corners} Marker points
 * @param width The width of the image
 * @param height The height of the image
 * @param gmSession {gm.Session} The GammaCV session
 * @returns a gm.Tensor with the transformed image
 */
function removePerspectiveGammaCV(
    sourceTensor: gm.Tensor,
    corners: Corners,
    width: number,
    height: number,
    gmSession: gm.Session
) {
    const rect = cornersToRectGm(corners);

    const tTransform = new gm.Tensor('float32', [3, 1, 4]);
    gm.generateTransformMatrix(
        rect, // Rect on original image to be projected
        [height, width], // Output dimensions
        tTransform // Tensor to be filled
    );
    const operation = gm.perspectiveProjection(sourceTensor, tTransform, [height, width, 4]);

    const outputTensor = gm.tensorFrom(operation);
    if (!outputTensor) return sourceTensor;

    // Create and initialize the GammaCV session, to acquire GPU acceperation
    // Then run the perspective projection operation
    gmSession.init(operation);
    gmSession.runOp(operation, 0, outputTensor);

    return outputTensor;
}


/**
 * Warp the perspective of the image to match the area selected by the markers.
 * The provided canvas is then populated with the resulting image.
 *
 * @param corners {Corners} Marker points
 * @param imageUrl {string} The url of the image to extract the selected area from
 * @param width {number} The width of the image
 * @param height {number} The height of the image
 * @param canvas {HTMLCanvasElement} The output canvas
 * @returns void
 */
export async function extractSelectedArea(
    imageStore: CapturedImage,
    gmSession: gm.Session
) {
    // TODO: fix window resizing
    const { imageUrl, corners, imageProportions } = imageStore;
    const { width, height } = imageProportions;

    const canvas = gm.canvasCreate(width, height);

    // Apply the perspective transformation using the selected corner marker coords
    const sourceTensor = await gm.imageTensorFromURL(imageUrl, 'uint8', [height, width, 4]);
    const resultTensor = removePerspectiveGammaCV(sourceTensor, corners, width, height, gmSession);

    const preProcessedTensor = preProcessTensor(resultTensor, canvas, gmSession);

    gm.canvasFromTensor(canvas, preProcessedTensor);

    return canvas;
}