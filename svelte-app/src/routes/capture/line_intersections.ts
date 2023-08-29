import * as gm from 'gammacv';

// Hyperparameters to tweak
export const PARAMS = {
    HAS_PCLINE: true,
    MAX_LINES: 16
};

/**
 * Merges all line intersections and clusters them together.
 * @param input - The input tensor.
 * @param allLineContexts - An array of all line contexts.
 * @returns An array of four points with the largest base.
 */
function mergeIntersections(input: gm.Tensor, allLineContexts: gm.Line[]) {
    let points: [number, number, number, boolean][] = [];

    for (let i = 0; i < allLineContexts.length; i++) {
        for (let j = i + 1; j < allLineContexts.length; j++) {
            const intersection = gm.Line.Intersection(allLineContexts[i], allLineContexts[j]);
            let dAngle = Math.abs(allLineContexts[i].angle - allLineContexts[j].angle);
            if (dAngle > 90) {
                dAngle = 180 - dAngle;
            }

            if (
                intersection &&
                dAngle > 20 &&
                intersection[0] >= 0 &&
                intersection[1] >= 0 &&
                intersection[0] <= input.shape[1] &&
                intersection[1] <= input.shape[0]
            ) {
                //gm.canvasDrawCircle(outputCanvas, intersection, 2, 'rgba(255, 0, 0, 1.0)');
                points.push([intersection[0], intersection[1], 1, true]);
            }
        }
    }

    // Cluster the points
    let pointsNextGen: [number, number, number, boolean][] = [];
    let numberOfMerges = 0;
    const maxRange = 100;

    const MAX_LOOPS = 1000000;
    let loopCount = 0;

    do {
        loopCount++;

        pointsNextGen = [];
        numberOfMerges = 0;

        for (let i = 0; i < points.length; i++) {
            if (points[i][3] == false) continue;

            for (let j = 0; j < points.length; j++) {
                if (i == j) continue;

                // If these points lie closely enough together and are not part of another cluster thusfar
                // Merge them and add them to the new generation
                if (
                    Math.abs(points[i][0] - points[j][0]) <= maxRange &&
                    Math.abs(points[i][1] - points[j][1]) <= maxRange
                ) {
                    const pointCount = points[i][2] + points[j][2];
                    const avgx = (points[i][0] * points[i][2] + points[j][0] * points[j][2]) / pointCount;
                    const avgy = (points[i][1] * points[i][2] + points[j][1] * points[j][2]) / pointCount;
                    pointsNextGen.push([avgx, avgy, pointCount, true]);

                    points[i][3] = false;
                    points[j][3] = false;

                    numberOfMerges++;
                }
            }
        }

        for (let i = 0; i < points.length; i++) {
            if (points[i][3]) {
                pointsNextGen.push(points[i]);
            }
        }

        points = pointsNextGen;
    } while (numberOfMerges > 0 && loopCount <= MAX_LOOPS);

    // Choose the four points with the largest base
    points = points.sort((b, a) => a[2] - b[2]).slice(0, 4);

    // for (let i = 0; i < points.length; i++) {
    //     gm.canvasDrawCircle(outputCanvas, [points[i][0], points[i][1]], 10, 'rgba(0, 0, 255, 1.0)');
    // }

    return points;
}

/**
     * Draw the lines on the canvas
     * @param input - Input tensor, before PCLines
     * @param output - Output tensor, after PCLines
     */
export function calculateIntersectionPoints(input: gm.Tensor, output: gm.Tensor) {

    let lines: [number, number, number][] = [];
    const maxP = Math.max(input.shape[0], input.shape[1]);

    for (let i = 0; i < output.size / 4; i += 1) {
        const y = Math.floor(i / output.shape[1]);
        const x = i - y * output.shape[1];
        const value = output.get(y, x, 0);
        const x0 = output.get(y, x, 1);
        const y0 = output.get(y, x, 2);

        if (value > 0.0) {
            lines.push([value, x0, y0]);
        }
    }

    lines = lines.sort((b, a) => a[0] - b[0]); // Sort by relevance
    lines = lines.slice(0, PARAMS.MAX_LINES); // Pick the N most relevant lines

    const allLineContexts: gm.Line[] = [];

    for (let i = 0; i < lines.length; i += 1) {
        const lineContext = new gm.Line();
        lineContext.fromParallelCoords(
            lines[i][1],
            lines[i][2],
            input.shape[1],
            input.shape[0],
            maxP,
            maxP / 2
        );

        // Gather all line contexts inside one array
        allLineContexts.push(lineContext);

        //gm.canvasDrawLine(outputCanvas, lineContext, 'rgba(0, 255, 0, 1.0)');
    }

    //
    // APPROACH: MERGING INTERSECTIONS
    //

    return []

    // return mergeIntersections(input, allLineContexts);
}