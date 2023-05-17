export interface Topic {
	id: number | string;
	title: string;
	url: string;
}

export interface ApiResponse {
	message?: string;
	status?: string;
}

export interface CustomError {
	error?: string;
	id?: number;
}

export interface RegenerateTokenRequestBody {
	email: string;
}

export interface LoginRequestBody extends RegenerateTokenRequestBody {
	password: string;
}

export interface RegisterRequestBody extends LoginRequestBody {
	first_name: string;
	last_name: string;
}
export interface PasswordChange {
	token: string;
	password: string;
}

interface UserProfile {
	id: string;
	user_id: string;
	phone_number: string | null;
	birth_date: string | null;
	github_link: string | null;
}

export interface User {
	email: string;
	first_name: string;
	last_name: string;
	id: string;
	is_staff: boolean;
	thumbnail: string;
	is_superuser: boolean;
	profile: UserProfile;
}

type Status = 'IDLE' | 'LOADING' | 'NAVIGATING';

export interface Loading {
	status: Status;
	message: string;
}

export interface SeriesAndArticles {
	id: string;
	name: string;
	image: string;
	articles: Array<Topic>;
}
