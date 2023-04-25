import type { Topic } from './types';

export const BASE_API_URI = import.meta.env.DEV
	? import.meta.env.VITE_BASE_API_URI_DEV
	: import.meta.env.VITE_BASE_API_URI_PROD;

export const danceEmoji = '💃';
export const angryEmoji = '😠';
export const sadEmoji = '😔';
export const happyEmoji = '😊';
export const thinkingEmoji = '🤔';
export const eyesRoll = '🙄';

export const redColor = '#dc3545';
export const greenColor = '#198754';
export const yellowColor = '#ffc107';
export const cyanColor = '#0dcaf0';

export const topics: Array<Topic> = [
	{
		id: 1,
		title: 'Backend Introduction',
		url: 'https://dev.to/sirneij/full-stack-authentication-system-using-rust-actix-web-and-sveltekit-1cc6'
	},
	{
		id: 2,
		title: 'Database and Redis Configuration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-db-and-redis-config-38fp'
	},
	{
		id: 3,
		title: 'User Registration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-user-registration-580h'
	},
	{
		id: 4,
		title: 'User session, Login and Logout',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-login-and-logout-1eb9'
	},
	{
		id: 5,
		title: 'CORS and Frontend Integration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-login-and-logout-1eb9'
	}
];
