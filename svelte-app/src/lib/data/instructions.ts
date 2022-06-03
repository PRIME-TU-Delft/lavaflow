export interface Instruction {
	title: string;
	description: string;
}

/**
 * List of requirements for the best experience with scanning the contour lines and targets
 */
export const instructions: Instruction[] = [
	{
		// TODO: add download link to template
		title: '3 marker visible',
		description: 'To recognize the level curves, we need to have the 3 markers fully visible'
	},
	{
		title: 'Curves should have sufficient margin',
		description:
			'The curves should have sufficient margin between them, overlapping or close lines are hard to interpret'
	},
	{
		title: 'Markers shoud not be too far or close',
		description:
			'Scanning works best from a distance of 0.5 meters from the piece of paper. For best the best result try to scan the paper as close to parallel as possible'
	},
	{
		title: 'Drawing with dark pen',
		description:
			'To recognise your level curves, we prefer a dark and thic pen. Using a lighter pen will result in inaccurate results'
	},
	{
		title: 'Propper lighting',
		description:
			'Level curves are hard to distinguish from background in a low light setting. Try to use natual daylight or a propper ceiling light for best results.'
	},
	{
		title: 'Close your curves',
		description: 'To avoid false readings, close your curves as much as possible'
	},
	{
		title: 'Only one paper should be visible',
		description: 'To avoid false readings, only one paper should be visible at a time'
	}
];
