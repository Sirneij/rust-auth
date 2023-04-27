import { uuidv4 } from './helpers/uuid.generator';
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

export const topics: Array<Topic> = [
	{
		id: uuidv4(),
		title: 'Backend Introduction',
		url: 'https://dev.to/sirneij/full-stack-authentication-system-using-rust-actix-web-and-sveltekit-1cc6'
	},
	{
		id: uuidv4(),
		title: 'Database and Redis Configuration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-db-and-redis-config-38fp'
	},
	{
		id: uuidv4(),
		title: 'User Registration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-user-registration-580h'
	},
	{
		id: uuidv4(),
		title: 'User session, Login and Logout',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-login-and-logout-1eb9'
	},
	{
		id: uuidv4(),
		title: 'CORS and Frontend Integration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-cors-and-frontend-integration-2j0h'
	},
	{
		id: uuidv4(),
		title: 'Log in/out, Dockerize and Deploy on fly.io',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-log-inout-dockerize-and-deploy-on-flyio-58pc'
	}
];
