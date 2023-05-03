import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { BASE_API_URI } from '$lib/utils/constant';
import type { CustomError } from '$lib/utils/types';
import { isValidPasswordMedium } from '$lib/utils/helpers/input.validation';
import { isEmpty } from '$lib/utils/helpers/test.object.empty';

export const load: PageServerLoad = async () => {
	return {};
};

export const actions: Actions = {
	default: async ({ fetch, request }) => {
		const formData = await request.formData();
		const password = String(formData.get('password'));
		const confirmPassword = String(formData.get('confirm_password'));
		const token = String(formData.get('token'));

		// Some validations
		const fieldsError: Record<string, string> = {};
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

		const requestInitOptions: RequestInit = {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ token: token, password: password })
		};

		const res = await fetch(
			`${BASE_API_URI}/users/password-change/change-user-password/`,
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
		throw redirect(302, `/auth/login?message=${response.message}`);
	}
};
