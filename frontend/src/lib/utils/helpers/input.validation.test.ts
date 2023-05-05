import { test, expect } from 'vitest';
import { isValidEmail, isValidPasswordMedium, isValidPasswordStrong } from './input.validation';

test('test isValidEmail', () => {
	let email = 'good@email.com';
	expect(isValidEmail(email)).toEqual(true);

	email = 'bad@email';
	expect(isValidEmail(email)).toEqual(false);
});

test('test isValidPasswordStrong', () => {
	let password = '123456Data@gmail.Com';
	expect(isValidPasswordStrong(password)).toEqual(true);

	password = 'badpassword';
	expect(isValidEmail(password)).toEqual(false);
});
test('test isValidPasswordMedium', () => {
	let password = '123456Data';
	expect(isValidPasswordMedium(password)).toEqual(true);

	password = 'badpassword';
	expect(isValidEmail(password)).toEqual(false);
});
