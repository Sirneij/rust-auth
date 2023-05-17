export const BASE_API_URI = import.meta.env.DEV
	? import.meta.env.VITE_BASE_API_URI_DEV
	: import.meta.env.VITE_BASE_API_URI_PROD;
export const BASE_SERIES_API_URI = import.meta.env.DEV
	? import.meta.env.VITE_BASE_SERIES_API_URI_DEV
	: import.meta.env.VITE_BASE_SERIES_API_URI_PROD;

export const IMAGE_UPLOAD_SIZE = ~~import.meta.env.VITE_IMAGE_UPLOAD_SIZE || 70;
export const HIGHEST_IMAGE_UPLOAD_SIZE = IMAGE_UPLOAD_SIZE * 1024;

export const danceEmoji = '💃';
export const angryEmoji = '😠';
export const sadEmoji = '😔';
export const happyEmoji = '😊';
export const thinkingEmoji = '🤔';
export const eyesRoll = '🙄';
