import type {
	ApiResponse,
	CustomError,
	LoginRequestBody,
	PasswordChange,
	RegenerateTokenRequestBody,
	RegisterRequestBody,
	User
} from '../types';

/**
 * Handle all POST-related requests.
 * @file lib/utils/requests/post.requests.ts
 * @param {typeof fetch} sveltekitFetch - Fetch object from sveltekit
 * @param {typeof fetch} url - The URL whose resource will be fetched.
 * @param {LoginRequestBody |  RegisterRequestBody | RegenerateTokenRequestBody | FormData |undefined} body - Body of the POST request
 * @param {RequestCredentials} [credentials='omit'] - Request credential. Defaults to 'omit'.
 * @param {'POST' | 'PUT' | 'PATCH' | 'DELETE'} [method='POST'] - Request method. Defaults to 'POST'.
 */
export const post = async (
	sveltekitFetch: typeof fetch,
	url: string,
	body:
		| LoginRequestBody
		| RegisterRequestBody
		| RegenerateTokenRequestBody
		| FormData
		| PasswordChange
		| undefined,
	credentials: RequestCredentials = 'omit',
	method: 'POST' | 'PUT' | 'PATCH' | 'DELETE' = 'POST'
): Promise<[object, Array<CustomError>]> => {
	try {
		const headers = { 'Content-Type': '' };
		const requestInitOptions: RequestInit = {
			method: method,
			mode: 'cors',
			credentials: credentials
		};
		if (!(body instanceof FormData)) {
			headers['Content-Type'] = 'application/json';
			requestInitOptions['headers'] = headers;
			requestInitOptions.body = JSON.stringify(body);
		} else if (body instanceof FormData) {
			headers['Content-Type'] = 'multipart/form-data';
			requestInitOptions['body'] = body;
		} else if (body === undefined && method !== 'DELETE') {
			const errors: Array<CustomError> = [
				{ error: 'Unless you are performing DELETE operation, you must have a body.', id: 0 }
			];
			return [{}, errors];
		}

		const res = await sveltekitFetch(url, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors: Array<CustomError> = [];

			errors.push({ error: response.error, id: 0 });

			return [{}, errors];
		}

		const res_json = await res.json();

		let response: ApiResponse | User;

		if (res_json['message']) {
			response = { message: res_json['message'], status: res_json['status'] };
		} else {
			response = {
				id: res_json['id'],
				email: res_json['email'],
				first_name: res_json['first_name'],
				last_name: res_json['last_name'],
				is_staff: res_json['is_staff'],
				thumbnail: res_json['thumbnail'],
				is_superuser: res_json['is_superuser'],
				profile: {
					id: res_json['profile']['id'],
					user_id: res_json['profile']['user_id'],
					phone_number: res_json['profile']['phone_number'],
					birth_date: res_json['profile']['birth_date'],
					github_link: res_json['profile']['github_link']
				}
			};
		}

		return [response, []];
	} catch (error) {
		console.error(`Error outside: ${error}`);
		const err = `${error}`;
		const errors: Array<CustomError> = [
			{ error: 'An unknown error occurred.', id: 0 },
			{ error: err, id: 1 }
		];
		return [{}, errors];
	}
};
