/**
 * This function takes a canvas with a removed perspective and converts it into a 2D array representing the image
 * The array has 100 rows and 100 columns, and each value represents the fraction of the pixels in that cell that are white
 * (0 is completely black, 1 is completely white)
 * The resolution of the grid can be changed by changing the resolution variable
 * @param perspectiveRemovedCanvas - The canvas with the perspective removed
 * @param resolution - The resolution of the grid (default 100)
 * @returns A 2D array representing the image
 */
export function imageToGrid(
	perspectiveRemovedCanvas: HTMLCanvasElement,
	resolution = 100
): number[][] {
	// The dx and dy values are used to split the image into grid cells
	// dx is the width of each cell, and dy is the height
	const dx = Math.floor(perspectiveRemovedCanvas.width / resolution);
	const dy = Math.floor(perspectiveRemovedCanvas.height / resolution);

	// Get the 2D context from the canvas
	// NOTE: willReadFrequently is set to true because this function will be called many times
	const context = perspectiveRemovedCanvas.getContext('2d', {
		willReadFrequently: true
	});

	// If the context doesn't exist, return an empty array
	if (!context) return [];

	// Create an empty array to store the grid
	const grid: number[][] = [];

	// Loop through each row
	for (let x = 0; x < resolution; x++) {
		// Create an empty array to store the row
		const row: number[] = [];

		// Loop through each column
		for (let y = 0; y < resolution; y++) {
			// Set the value of the cell to 0
			let gridValue = 0;

			// Loop through each pixel in the cell, and add 1 to the cell value for each white pixel
			for (let i = 0; i < dx; i++) {
				for (let j = 0; j < dy; j++) {
					const pixel = context.getImageData(x * dx + i, y * dy + j, 1, 1).data?.[0] || 0;

					if (pixel > 0) {
						gridValue++;
					}
				}
			}

			row.push(1 - gridValue / (dx * dy));
		}

		grid.push(row);
	}

	return grid;
}
