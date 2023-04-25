export interface Topic {
	id: number;
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

export interface User {
	email: string;
	first_name: string;
	last_name: string;
	id: string;
	is_staff: boolean;
	thumbnail: string;
	is_superuser: boolean;
}

type Status = 'IDLE' | 'LOADING' | 'NAVIGATING';

export interface Loading {
	status: Status;
	message: string;
}
