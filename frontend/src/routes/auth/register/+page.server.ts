import { redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { BASE_API_URI } from '$lib/utils/constant';
import type { CustomError, RegisterRequestBody } from '$lib/utils/types';
import { isValidEmail, isValidPasswordMedium } from '$lib/utils/helpers/input.validation';
import { isEmpty } from '$lib/utils/helpers/test.object.empty';

export const load: PageServerLoad = async ({ locals }) => {
	// redirect user if logged in
	if (locals.user) {
		throw redirect(302, '/');
	}
};

export const actions: Actions = {
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	register: async ({ request, fetch }) => {
		const formData = await request.formData();
		const email = String(formData.get('email'));
		const firstName = String(formData.get('first_name'));
		const lastName = String(formData.get('last_name'));
		const password = String(formData.get('password'));
		const confirmPassword = String(formData.get('confirm_password'));

		// Some validations
		const fieldsError: Record<string, string> = {};
		if (!isValidEmail(email)) {
			fieldsError.email = 'That email address is invalid.';
		}
		if (!isValidPasswordMedium(password)) {
			fieldsError.password =
				'Password is not valid. Password must contain six characters or more and has at least one lowercase and one uppercase alphabetical character or has at least one lowercase and one numeric character or has at least one uppercase and one numeric character.';
		}
		if (confirmPassword.trim() !== password.trim()) {
			fieldsError.confirmPassword = 'Password and confirm password do not match.';
		}

		if (!isEmpty(fieldsError)) {
			return fail(400, { fieldsError: fieldsError });
		}

		const registrationBody: RegisterRequestBody = {
			email,
			first_name: firstName,
			last_name: lastName,
			password
		};

		const apiURL = `${BASE_API_URI}/users/register/`;

		const requestInitOptions: RequestInit = {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(registrationBody)
		};

		const res = await fetch(apiURL, requestInitOptions);

		if (!res.ok) {
			const response = await res.json();
			const errors: Array<CustomError> = [];
			errors.push({ error: response.error, id: 0 });
			return fail(400, { errors: errors });
		}

		const response = await res.json();

		throw redirect(303, `/auth/confirming?message=${response.message}`);
	}
};
