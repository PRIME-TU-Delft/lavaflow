import inverseKernelGLSL from './inverseKernel.glsl?raw';
import * as gm from 'gammacv';

function inverseKernel(previous: gm.InputType) {
    const width = previous.shape[1];
    const height = previous.shape[0];

    return new gm.RegisterOperation('inverse')
        .Input('tSrc', 'uint8')
        .Output('uint8')
        .SetShapeFn(() => [height, width, 4])
        .LoadChunk('pickValue')
        .GLSLKernel(inverseKernelGLSL)
        .Compile({ tSrc: previous });
}

function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
    const height = input.shape[0];
    const width = input.shape[1];

    // Normalization: add contrast, make colors seem deeper
    let pipeline = gm.grayscale(input);
    pipeline = gm.erode(pipeline, [10, 10]);

    pipeline = gm.gaussianBlur(pipeline, 3, 1);
    pipeline = gm.sobelOperator(pipeline);
    pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);
    pipeline = gm.gaussianBlur(pipeline, 3, 3);
    pipeline = inverseKernel(pipeline);
    pipeline = gm.gaussianBlur(pipeline, 3, 3);
    pipeline = gm.threshold(pipeline, 0.9, 1);

    // pipeline = sharpen(pipeline, 32);

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