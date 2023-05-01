import { HIGHEST_IMAGE_UPLOAD_SIZE } from '../constant';

/**
 * Determine and nicely format the file size of an item.
 * @file lib/utils/helpers/image.file.size.ts
 * @param {number} num - The size of the file.
 * @returns {string} - The nicely formatted file size.
 */
export const returnFileSize = (num: number): [string, boolean] => {
	let returnString = '';
	if (num < 1024) {
		returnString = `${num} bytes`;
	} else if (num >= 1024 && num < 1048576) {
		returnString = `${(num / 1024).toFixed(1)} kB`;
	} else if (num >= 1048576) {
		returnString = `${(num / 1048576).toFixed(1)} MB`;
	}
	return [returnString, num < HIGHEST_IMAGE_UPLOAD_SIZE];
};
