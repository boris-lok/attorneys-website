import { redirect } from '@sveltejs/kit';

export const handle = async ({ event, resolve }) => {
	if (event.url.pathname.startsWith('/admin') && !event.url.pathname.includes('login')) {
		const user = JSON.parse(event.cookies.get('user') || '{}');

		if (!(
			user && 'user_id' in user && 'username' in user && 'token' in user
		)) {
			throw redirect(302, '/admin/login');
		}
	}

	return await resolve(event);
};