import { UnitTestExport as detectCurves } from './detectCurves';

test('getLevels (basic)', () => {
	const hierarchy = [-1, 0, 0, 1, 3, 4, 2];
	expect(detectCurves.getLevels(hierarchy)).toEqual([0, 1, 1, 2, 3, 4, 2]);
});

test('getLevels (weird hierarchy ordering)', () => {
	const hierarchy = [-1, 0, 0, 1, 3, 4, 2, 0, -1, 6, 1];
	expect(detectCurves.getLevels(hierarchy)).toEqual([0, 1, 1, 2, 3, 4, 2, 1, 0, 3, 2]);
});
