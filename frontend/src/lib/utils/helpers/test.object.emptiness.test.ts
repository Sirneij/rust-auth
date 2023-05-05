import { test, expect } from 'vitest';
import { isEmpty } from './test.object.empty';

test('test isEmpty', () => {
	let obj = {};
	expect(isEmpty(obj)).toEqual(true);

	obj = {
		password: '123456Data'
	};
	expect(isEmpty(obj)).toEqual(false);
});
