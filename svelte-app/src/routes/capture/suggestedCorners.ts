import * as gm from 'gammacv';


type Vector2D = { x: number, y: number };
type Corners = [Vector2D, Vector2D, Vector2D, Vector2D]

type Lines = [number, number, number][]

function getLines(input: gm.Tensor, output: gm.Tensor): gm.Line[] {
    const MAX_LINES = 16; // Hyperparameter for the maximum number of lines to detect

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
    }

    return allLineContexts;
}

type IntersectionPoint = {
    x: number,
    y: number,
    represents: number,
    isPartOfMerge: boolean
}

function getIntersectionPoints(input: gm.Tensor, allLineContexts: gm.Line[]) {
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
            }
        }
    }

    return points;
}

// TODO:
function clusterPoints(allIntersectionPoints: IntersectionPoint[]) {
    let pointsNextGen: [number, number, number, boolean][] = [];
    let numberOfMerges = 0;
    const maxRange = 100;

    do {
        pointsNextGen = [];
        numberOfMerges = 0;

        for (let i = 0; i < points.length; i++) {
            if (points[i][3] == false) continue;

            for (let j = i; j < points.length; j++) {
                if (i == j) continue;

                if (points[j][3] == false) continue;

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

        console.log(points.length);
    } while (numberOfMerges > 0);
}

export default function suggestedCorners(input: gm.Tensor, output: gm.Tensor): Corners {
    const lines = getLines(input, output);

    const allIntersectionPoints = getIntersectionPoints(input, lines);

    let corners = clusterPoints(allIntersectionPoints);


}


