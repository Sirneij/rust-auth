import type { Topic } from './types';

export const BASE_API_URI = import.meta.env.DEV
	? import.meta.env.VITE_BASE_API_URI_DEV
	: import.meta.env.VITE_BASE_API_URI_PROD;

export const IMAGE_UPLOAD_SIZE = ~~import.meta.env.VITE_IMAGE_UPLOAD_SIZE || 70;
export const HIGHEST_IMAGE_UPLOAD_SIZE = IMAGE_UPLOAD_SIZE * 1024;

export const danceEmoji = '💃';
export const angryEmoji = '😠';
export const sadEmoji = '😔';
export const happyEmoji = '😊';
export const thinkingEmoji = '🤔';
export const eyesRoll = '🙄';

export const topics: Array<Topic> = [
	{
		id: crypto.randomUUID(),
		title: 'Backend Introduction',
		url: 'https://dev.to/sirneij/full-stack-authentication-system-using-rust-actix-web-and-sveltekit-1cc6'
	},
	{
		id: crypto.randomUUID(),
		title: 'Database and Redis Configuration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-db-and-redis-config-38fp'
	},
	{
		id: crypto.randomUUID(),
		title: 'User Registration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-user-registration-580h'
	},
	{
		id: crypto.randomUUID(),
		title: 'User session, Login and Logout',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-login-and-logout-1eb9'
	},
	{
		id: crypto.randomUUID(),
		title: 'CORS and Frontend Integration',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-cors-and-frontend-integration-2j0h'
	},
	{
		id: crypto.randomUUID(),
		title: 'Log in/out, Dockerize and Deploy on fly.io',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-log-inout-dockerize-and-deploy-on-flyio-58pc'
	},
	{
		id: crypto.randomUUID(),
		title: 'Log in/out, Dockerize and Deploy on fly.io',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-log-inout-dockerize-and-deploy-on-flyio-58pc'
	},
	{
		id: crypto.randomUUID(),
		title: 'File upload to AWS S3, Profile Update',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-file-upload-to-aws-s3-profile-update-3b2a'
	},
	{
		id: crypto.randomUUID(),
		title: 'User Profile Update UI',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-user-profile-update-ui-4f78'
	},
	{
		id: crypto.randomUUID(),
		title: 'Enhancing the frontend application with form actions',
		url: 'https://dev.to/sirneij/authentication-system-using-rust-actix-web-and-sveltekit-enhancing-the-frontend-application-with-form-actions-3d10'
	}
];
