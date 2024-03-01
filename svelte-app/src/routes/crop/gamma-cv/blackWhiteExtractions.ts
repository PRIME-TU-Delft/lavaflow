import * as gm from 'gammacv';
import whiteBalanceKernel from "./whiteBalance.glsl?raw"

function whiteBalance(input: gm.Operation | gm.Tensor) {
    const [HEIGHT, WIDTH] = input.shape;

    return new gm.RegisterOperation("whiteBalance")
        .Input("tSrc", "uint8")
        .Output("uint8")
        .SetShapeFn(() => [HEIGHT, WIDTH, 4])
        .LoadChunk("pickValue")
        .GLSLKernel(whiteBalanceKernel)
        .Compile({ tSrc: input });
}

function BW_pipeline(input: gm.Tensor<gm.TensorDataView>) {
    // Normalization: add contrast, make colors seem deeper
    let pipeline = gm.grayscale(input);
    pipeline = gm.erode(pipeline, [10, 10]);
    pipeline = gm.gaussianBlur(pipeline, 3, 1);
    pipeline = whiteBalance(pipeline);
    pipeline = gm.adaptiveThreshold(pipeline, 200, 0.95);

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