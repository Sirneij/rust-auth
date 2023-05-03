import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BASE_API_URI } from '$lib/utils/constant';
import type { CustomError } from '$lib/utils/types';
import { isValidEmail } from '$lib/utils/helpers/input.validation';
import { isEmpty } from '$lib/utils/helpers/test.object.empty';

export const load: PageServerLoad = async () => {
	return {};
};

export const actions: Actions = {
	default: async ({ fetch, request }) => {
		const formData = await request.formData();
		const email = String(formData.get('email'));

		// Some validations
		const fieldsError: Record<string, string> = {};
		if (!isValidEmail(email)) {
			fieldsError.email = 'That email address is invalid.';
		}

		if (!isEmpty(fieldsError)) {
			return fail(400, { fieldsError: fieldsError });
		}

		const requestInitOptions: RequestInit = {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ email: email })
		};

		const res = await fetch(
			`${BASE_API_URI}/users/password-change/request-password-change/`,
			requestInitOptions
		);

		if (!res.ok) {
			const response = await res.json();
			const errors: Array<CustomError> = [];
			errors.push({ error: response.error, id: 0 });
			return fail(400, { errors: errors });
		}

		const response = await res.json();

		// redirect the user
		throw redirect(302, `/auth/confirming?message=${response.message}`);
	}
};
