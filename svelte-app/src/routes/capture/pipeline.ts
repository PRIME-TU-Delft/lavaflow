
import * as gm from 'gammacv';

/**
 * Define the pipeline for the image processing
 * @param input Input tensor
 */
function getPipeLine(input: gm.Tensor, HAS_PCLINE = true) {
    let pipeline = gm.dilate(input, [12, 12]);
    pipeline = gm.grayscale(pipeline);
    pipeline = gm.gaussianBlur(pipeline, 3, 3);
    pipeline = gm.sobelOperator(pipeline);
    pipeline = gm.cannyEdges(pipeline, 0.25, 0.75);

    if (HAS_PCLINE) {
        pipeline = gm.pcLines(pipeline);
    }
    return pipeline;
}

export default getPipeLine