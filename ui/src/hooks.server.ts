import { redirect } from '@sveltejs/kit'
import type { Credential } from '$lib/services/user.service'

export const handle = async ({ event, resolve }) => {
    const user: Partial<Credential> = JSON.parse(
        event.cookies.get('user') || '{}',
    )

    if (
        event.url.pathname.startsWith('/admin') &&
        !event.url.pathname.includes('login')
    ) {
        if (!user.token) {
            throw redirect(302, '/admin/login')
        }

        if (!checkRole(user.roles ?? [], 'admin')) {
            throw redirect(302, '/permission_denied')
        }
    }

    if (event.url.pathname.startsWith('/b')) {
        if (!user.token) {
            throw redirect(302, '/admin/login')
        }

        if (
            !checkRole(user.roles ?? [], 'admin') &&
            !checkRole(user.roles ?? [], 'lawyer')
        ) {
            throw redirect(302, '/permission_denied')
        }
    }

    return await resolve(event)
}

// check the role is included by given roles.
//
// return true, if exist
// return false, if not exist
const checkRole = (roles: string[], role: string) => {
    return roles.some((e) => e.toLowerCase() === role)
}
