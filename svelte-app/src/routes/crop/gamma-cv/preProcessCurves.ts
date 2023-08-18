import * as gm from 'gammacv';

function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
    // Normalization: add contrast, make colors seem deeper
    let pipeline = gm.norm(input, 'l2');
    // Erosion: erode into rectangles of shape 2x2 (best to see for yourself: https://gammacv.com/examples/erode)
    pipeline = gm.erode(pipeline, [2, 2]);
    // Adaptive Threshold: Black/white - make pixels black if they pass the threshold 20 within a certain box of size 10
    // (best to see for yourself: https://gammacv.com/examples/adaptive_threshold)
    pipeline = gm.adaptiveThreshold(pipeline, 10, 35);
    // Gaussian Blur: remove sharp edges
    pipeline = gm.gaussianBlur(pipeline, 3, 1);
    // Make the lines a bit thinner so the result from opencv's getContours is better
    pipeline = gm.threshold(pipeline, 0.3);

    return pipeline;
}

export function preProcessTensor(input: gm.Tensor, canvas: HTMLCanvasElement, gmSession: gm.Session): gm.Tensor {

    const pipeline = BW_pipeline(input);

    const outputTensor = gm.tensorFrom(pipeline);
    if (!outputTensor) throw new Error('Could not create output tensor');

    // Create and initialize the GammaCV session, to acquire GPU acceperation
    // Then run the perspective projection operation
    gmSession.init(pipeline);

    // Draw the output on the canvas
    gmSession.runOp(pipeline, 0, outputTensor);

    return outputTensor;
}