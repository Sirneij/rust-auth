import { test, expect } from 'vitest';
import { returnFileSize } from './image.file.size';

test('test image.file.size.ts file', () => {
	let fileSize = 20;
	expect(returnFileSize(fileSize)).toEqual([`${fileSize} bytes`, true]);

	fileSize = 1240;
	expect(returnFileSize(fileSize)).toEqual(['1.2 kB', true]);

	fileSize = 1048580;
	expect(returnFileSize(fileSize)).toEqual(['1.0 MB', false]);
});
