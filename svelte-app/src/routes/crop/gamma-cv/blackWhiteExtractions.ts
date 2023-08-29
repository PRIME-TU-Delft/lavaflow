import * as gm from 'gammacv';
function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
    const height = input.shape[0];
    const width = input.shape[1];

    // Normalization: add contrast, make colors seem deeper
    let pipeline = gm.grayscale(input);
    pipeline = gm.erode(pipeline, [10, 10]);
    pipeline = gm.gaussianBlur(pipeline, 3, 1);
    pipeline = gm.threshold(pipeline, 0.75);

    return pipeline;
}

export function applyBlackAndWhiteFilterGammaCV(sourceTensor: gm.Tensor, gmSession: gm.Session) {
    const operation = BW_pipeline(sourceTensor);

    const outputTensor = gm.tensorFrom(operation);
    if (!outputTensor) return sourceTensor;

    // Create and initialize the GammaCV session, to acquire GPU acceperation
    // Then run the perspective projection operation
    gmSession.init(operation);
    gmSession.runOp(operation, 0, outputTensor);

    return outputTensor;
}