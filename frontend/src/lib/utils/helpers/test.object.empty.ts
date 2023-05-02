/**
 * Test whether or not an object is empty.
 * @param {Record<string, string>} obj - The object to test
 * @returns `true` or `false`
 */

export function isEmpty(obj: Record<string, string>) {
	for (const _i in obj) {
		return false;
	}
	return true;
}
