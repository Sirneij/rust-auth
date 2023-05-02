import { redirect, type Actions, fail } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { BASE_API_URI, IMAGE_UPLOAD_SIZE } from '$lib/utils/constant';
import type { CustomError, User } from '$lib/utils/types';
import { returnFileSize } from '$lib/utils/helpers/image.file.size';

export const load: PageServerLoad = async ({ locals, params }) => {
	// redirect user if not logged in
	if (!locals.user) {
		throw redirect(302, `/auth/login?next=/auth/about/${params.id}`);
	}
};

export const actions: Actions = {
	/**
	 *
	 * @param request - The request object
	 * @param fetch - Fetch object from sveltekit
	 * @param cookies - SvelteKit's cookie object
	 * @param locals - The local object, housing current user
	 * @returns Error data or redirects user to the home page or the previous page
	 */
	updateUser: async ({ request, fetch, cookies, locals }) => {
		const formData = await request.formData();

		// Some validations
		const errors: Array<CustomError> = [];

		// Ensure that the file is not too big
		if (formData.get('thumbnail')) {
			const file = formData.get('thumbnail') as File;
			if (file.size <= 0 || file.name === '' || file.length <= 0) {
				formData.delete('thumbnail');
			} else {
				const [size, isValid] = returnFileSize(file.size);

				if (!isValid) {
					errors.push({
						id: Math.floor(Math.random() * 100),
						error: `Image size ${size} is too large. Please keep it below ${IMAGE_UPLOAD_SIZE}kB.`
					});
					formData.delete('thumbnail');
				}
			}
		}
		// Ensure that first_name is different from the current one
		if (formData.get('first_name')) {
			const firstName = formData.get('first_name');
			if (firstName === locals.user.first_name || firstName === '') {
				formData.delete('first_name');
			}
		}
		// Ensure that last_name is different from the current one
		if (formData.get('last_name')) {
			const lastName = formData.get('last_name');
			if (lastName === locals.user.last_name || lastName === '') {
				formData.delete('last_name');
			}
		}

		if (errors.length > 0) {
			return fail(400, { errors: errors });
		}

		const apiURL = `${BASE_API_URI}/users/update-user/`;

		const res = await fetch(apiURL, {
			method: 'PATCH',
			headers: {
				Cookie: `sessionid=${cookies.get('id')}`
			},
			body: formData
		});

		if (!res.ok) {
			const response = await res.json();
			const errors: Array<CustomError> = [];
			errors.push({ error: response.error, id: Math.floor(Math.random() * 100) });
			return fail(400, { errors: errors });
		}

		const response = (await res.json()) as User;

		locals.user = response;

		throw redirect(303, `/auth/about/${response.id}`);
	}
};
