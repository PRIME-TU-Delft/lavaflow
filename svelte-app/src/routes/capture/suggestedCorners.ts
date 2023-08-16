import * as gm from 'gammacv';

export type Vector2D = { x: number, y: number };
export type Corners = { topLeft: Vector2D, topRight: Vector2D, bottomLeft: Vector2D, bottomRight: Vector2D }
type DefualtCorners = { defaultTopLeft: Vector2D, defaultTopRight: Vector2D, defaultBottomLeft: Vector2D, defaultBottomRight: Vector2D }

type Lines = [number, number, number][]

/**
 * Get the lines from the output of the pipeline
 * @param input - The input image as a tensor
 * @param output - The output of the pipeline transformations as a tensor
 * @param debugCanvas - The canvas to draw debug information on
 * @param MAX_LINES - The maximum number of lines to detect (default: 16)
 * @returns a list of lines
 */
function getLines(input: gm.Tensor, output: gm.Tensor, debugCanvas?: HTMLCanvasElement, MAX_LINES = 16): gm.Line[] {
    let lines: Lines = [];
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
    lines = lines.slice(0, MAX_LINES); // Pick the N most relevant lines

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

        if (debugCanvas) gm.canvasDrawLine(debugCanvas, lineContext, 'rgba(0, 255, 0, 1.0)');
    }

    return allLineContexts;
}

type IntersectionPoint = {
    x: number,
    y: number,
    represents: number,
    isPartOfMerge: boolean
}

/**
 * Get all intersection points from a list of lines
 * @param input - The input image as a tensor
 * @param allLineContexts - A list of line contexts
 * @returns a list of intersection points
 */
function getIntersectionPoints(input: gm.Tensor, allLineContexts: gm.Line[], debugCanvas?: HTMLCanvasElement) {
    const points: IntersectionPoint[] = [];

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
                points.push({ x: intersection[0], y: intersection[1], represents: 1, isPartOfMerge: true });

                if (debugCanvas) {
                    gm.canvasDrawCircle(debugCanvas, intersection, 2, 'rgba(255, 0, 0, 1.0)');
                }
            }
        }
    }

    return points;
}

/**
 * Cluster all intersection points together that lie closely enough together
 * @param allIntersectionPoints - all intersection points
 * @returns 
 */
function clusterPoints(allIntersectionPoints: IntersectionPoint[]) {
    let pointsNextGen: IntersectionPoint[] = [];
    let numberOfMerges = 0;
    const MAX_RANGE = 100; // TODO: Make dependent on resolution / make responsive - Hyperparameter for the maximum range between two points to be considered part of the same cluster

    do {
        pointsNextGen = [];
        numberOfMerges = 0;

        for (let i = 0; i < allIntersectionPoints.length; i++) {
            if (allIntersectionPoints[i].isPartOfMerge == false) continue;

            for (let j = i + 1; j < allIntersectionPoints.length; j++) {
                const pointsI = allIntersectionPoints[i];
                const pointsJ = allIntersectionPoints[j];

                if (allIntersectionPoints[j].isPartOfMerge == false) continue;

                // If these points lie closely enough together and are not part of another cluster thusfar
                // Merge them and add them to the new generation
                if (
                    Math.abs(pointsI.x - pointsJ.x) <= MAX_RANGE &&
                    Math.abs(pointsI.y - pointsJ.y) <= MAX_RANGE
                ) {
                    const pointCount = pointsI.represents + pointsJ.represents;
                    const avgx = (pointsI.x * pointsI.represents + pointsJ.x * pointsJ.represents) / pointCount;
                    const avgy = (pointsI.y * pointsI.represents + pointsJ.y * pointsJ.represents) / pointCount;
                    pointsNextGen.push({ x: avgx, y: avgy, represents: pointCount, isPartOfMerge: true });

                    pointsI.isPartOfMerge = false;
                    pointsJ.isPartOfMerge = false;

                    numberOfMerges++;
                }
            }
        }

        allIntersectionPoints = [...pointsNextGen, ...allIntersectionPoints.filter((point) => point.isPartOfMerge)]

    } while (numberOfMerges > 0);

    return allIntersectionPoints;
}

/**
 * Method that performs argmin on a certain given property
 * @param prop 
 * @param points 
 * @param inclusionRequirement 
 * @param fallback 
 * @returns 
 */
function minimize(
    prop: (x: number, y: number) => number,
    points: Vector2D[],
    inclusionRequirement: (x: number, y: number) => boolean = (x, y) => true,
    fallback: Vector2D
): Vector2D {
    let min: number | undefined = undefined;
    let minArg: Vector2D | undefined = undefined;

    for (let point of points) {
        if ((min == undefined || prop(point.x, point.y) < min) && inclusionRequirement(point.x, point.y)) {
            min = prop(point.x, point.y);
            minArg = point;
        }
    }

    // Since we allow posing additional inclusion requirements, it's possible we don't find
    // any point with minimal property. In that case, minArg = undefined. 
    // So, we return the fallback
    return minArg || fallback;
}

/**
 * Get the default corners of the image
 * @param input - The input image as a tensor
 * @returns the default corners
 */
export function defualtCorners(input: gm.Tensor): DefualtCorners {
    const canvasWidth = input.shape[1];
    const canvasHeight = input.shape[0];

    // Start by introducing a default marker in each corner
    let defaultBorderOffsetX = canvasWidth / 4;
    let defaultBorderOffsetY = canvasHeight / 4;

    let defaultTopLeft = { x: defaultBorderOffsetX, y: defaultBorderOffsetY };
    let defaultTopRight = { x: canvasWidth - defaultBorderOffsetX, y: defaultBorderOffsetY };
    let defaultBottomLeft = { x: defaultBorderOffsetX, y: canvasHeight - defaultBorderOffsetY };
    let defaultBottomRight = { x: canvasWidth - defaultBorderOffsetX, y: canvasHeight - defaultBorderOffsetY };

    return { defaultTopLeft, defaultTopRight, defaultBottomLeft, defaultBottomRight };
}

function getCorners(input: gm.Tensor, intersections: Vector2D[], debugCanvas?: HTMLCanvasElement): Corners {
    const defaults = defualtCorners(input);
    const canvasWidth = input.shape[1];
    const canvasHeight = input.shape[0];
    const defaultValues = Object.values(defaults);

    // Top left is the point with smallest x and y
    // Find the marker closest to the top-left corner from all found intersections and the defaults
    let topLeft = minimize(
        (x: number, y: number) => {
            return x * y;
        },
        [...intersections, ...defaultValues],
        // Pose an additional requirement that the marker has to be in the top left quadrant
        (x: number, y: number) => x <= canvasWidth / 2 && y <= canvasHeight / 2,
        defaults.defaultTopLeft
    );

    // Top right is the point with largest x and smallest y
    // Find the marker closest to the top-right corner from all found intersections and the defaults,
    // exclude the marker we just assigned top-left!
    let topRight = minimize(
        (x: number, y: number) => {
            return (1 / x) * y;
        },
        [...intersections, ...defaultValues].filter((a) => a != topLeft),
        // Pose an additional requirement that the marker has to be in the top right quadrant
        (x: number, y: number) => x >= canvasWidth / 2 && y <= canvasHeight / 2,
        defaults.defaultTopRight
    );

    // Bottom left is the point with smallest x and largest y
    // Find the marker closest to the bottom-left corner from all found intersections and the defaults
    // exclude any markers we just assigned!
    let bottomLeft = minimize(
        (x: number, y: number) => {
            return (x * 1) / y;
        },
        [...intersections, ...defaultValues].filter((a) => a != topLeft && a != topRight),
        // Pose an additional requirement that the marker has to be in the top left quadrant
        (x: number, y: number) => x <= canvasWidth / 2 && y >= canvasHeight / 2,
        defaults.defaultBottomLeft
    );

    // Bottom right is the point with largest x and largest y
    // Find the marker closest to the bottom-right corner from all found intersections and the defaults
    // exclude any markers we just assigned!
    let bottomRight = minimize(
        (x: number, y: number) => {
            return ((1 / x) * 1) / y;
        },
        [...intersections, ...defaultValues].filter(
            (a) => a != topLeft && a != topRight && a != bottomLeft
        ),
        // Pose an additional requirement that the marker has to be in the top left quadrant
        (x: number, y: number) => x >= canvasWidth / 2 && y >= canvasHeight / 2,
        defaults.defaultBottomRight
    );

    if (debugCanvas) {
        gm.canvasFillCircle(debugCanvas, Object.values(topLeft), 12, 'rgba(255, 0, 255, 1.0)');
        gm.canvasFillCircle(debugCanvas, Object.values(topRight), 12, 'rgba(255, 0, 255, 1.0)');
        gm.canvasFillCircle(debugCanvas, Object.values(bottomLeft), 12, 'rgba(255, 0, 255, 1.0)');
        gm.canvasFillCircle(debugCanvas, Object.values(bottomRight), 12, 'rgba(255, 0, 255, 1.0)');
    }

    return { topLeft, topRight, bottomLeft, bottomRight };
}

/**
 * Will return the suggested corners of the image
 * @param input - The input image as a tensor
 * @param output - The output of the pipeline transformations as a tensor
 * @param debugCanvas - The canvas to draw debug information on
 * @returns the suggested corners
 */
export function suggestedCorners(input: gm.Tensor, output: gm.Tensor, debugCanvas?: HTMLCanvasElement): Corners {
    const lines = getLines(input, output, debugCanvas);

    const allIntersectionPoints = getIntersectionPoints(input, lines, debugCanvas);

    let clusters = clusterPoints(allIntersectionPoints);
    clusters.sort((b, a) => a.represents - b.represents); // Sort by relevance
    let intersections = clusters.map(cluster => ({ x: cluster.x, y: cluster.y }));

    return getCorners(input, intersections, debugCanvas);
}


