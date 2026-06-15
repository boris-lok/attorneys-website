import { requireAnyRole } from '$lib/server/auth'

export const load = async ({ locals, url }) => {
    const user = requireAnyRole(locals, 'admin', 'lawyer')
    return { user }
}
