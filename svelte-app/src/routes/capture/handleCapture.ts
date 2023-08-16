import * as gm from 'gammacv';
import getPipeline from './pipeline';
import { suggestedCorners } from './suggestedCorners';

export { type Corners, defualtCorners } from './suggestedCorners';

/**
 * Convert video feed to tensor (via a capture of a video frame in a canvas)
 * @param videoSource - HTMLVideoElement
 * @param canvas - HTMLCanvasElement
 * @param width - width of the video
 * @param height - height of the video
 * @returns 
 */
export async function videoToTensor(videoSource: HTMLVideoElement | undefined, canvas: HTMLCanvasElement) {
    if (!videoSource) throw new Error('Video source is undefined');
    const width = videoSource.videoWidth;
    const height = videoSource.videoHeight;

    canvas.width = width;
    canvas.height = height;
    const context = canvas.getContext('2d');
    if (!context) throw new Error('Could not get canvas context');

    context.drawImage(videoSource, 0, 0, width, height); // draw video frame to canvas
    const imageUrl = canvas.toDataURL('image/png'); // convert canvas to image
    const input = await gm.imageTensorFromURL(imageUrl, 'uint8', [height, width, 4], true); // convert image to tensor

    if (!input) throw new Error('Could not get input tensor');
    return { input, imageUrl };
}

/**
 * Exposed function to handle capture of a video frame and process it with GammaCV.
 * @param input gm.Tensor - input tensor
 * @param width number - width of the window
 * @param height number - height of the window
 */
export function handleCapture(input: gm.Tensor, gmSession: gm.Session, debugCanvas?: HTMLCanvasElement) {
    // 3. Define pipeline, initialize pipeline, allocate output tensor
    const pipeline = getPipeline(input); // Define pipeline
    gmSession.init(pipeline); // initialize pipeline
    const output = gm.tensorFrom(pipeline); // allocate output tensor
    if (!output) throw new Error('Could not get output tensor');

    // 4. run pipeline
    gmSession.runOp(pipeline, 0, output);

    return suggestedCorners(input, output, debugCanvas);
}


