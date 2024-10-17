export const handle = async ({ event, resolve }) => {
	console.log(event.request.headers.get('cookie'))
	return await resolve(event);
};