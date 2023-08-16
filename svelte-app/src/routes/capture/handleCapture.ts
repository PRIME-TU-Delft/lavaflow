import * as gm from 'gammacv';
import getPipeline from './pipeline';
import getSuggestedCorners from './suggestedCorners';

function videoToImageUrl(videoSource: HTMLVideoElement, canvas: HTMLCanvasElement, width: number, height: number) {
    canvas.width = width;
    canvas.height = height;
    const context = canvas.getContext('2d');
    if (!context) return;

    context.drawImage(videoSource, 0, 0, width, height);

    return canvas.toDataURL('image/png');
}

/**
 * Exposed function to handle capture of a video frame and process it with GammaCV.
 * 
 * @param videoSource HTMLVideoElement - used to get image url from video
 * @param canvas HTMLCanvasElement - used to get image url from video
 * @param width number - width of the window
 * @param height number - height of the window
 */
export default async function handleCapture(videoSource: HTMLVideoElement | undefined, canvas: HTMLCanvasElement, width: number, height: number, gmSession: gm.Session) {
    if (!videoSource) throw new Error('Video source is undefined');

    // 1. Get image url from video
    const imageUrl = videoToImageUrl(videoSource, canvas, width, height)
    if (!imageUrl) throw new Error('Could not get image url from video');

    // 2. Get image tensor from url
    const input = await gm.imageTensorFromURL(imageUrl, 'uint8', [height, width, 4], true);

    // 3. Define pipeline, initialize pipeline, allocate output tensor
    const pipeline = getPipeline(input); // Define pipeline
    gmSession.init(pipeline); // initialize pipeline
    const output = gm.tensorFrom(pipeline); // allocate output tensor
    if (!output) throw new Error('Could not get output tensor');

    // 4. run pipeline
    gmSession.runOp(pipeline, 0, output);

    return getSuggestedCorners();
}


