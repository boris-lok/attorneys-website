import { redirect } from '@sveltejs/kit'
import type { Credential } from '$lib/services/user.service'

type ValidateRoute = {
    start: string
    excludes: string[]
    roles: string[]
}

const validateRoute: ValidateRoute[] = [
    {
        start: '/admin',
        excludes: ['login'],
        roles: ['admin'],
    },
    {
        start: '/b',
        excludes: [],
        roles: ['admin', 'lawyer'],
    },
]

export const handle = async ({ event, resolve }) => {
    const user: Partial<Credential> = JSON.parse(
        event.cookies.get('user') || '{}',
    )

    for (const e of validateRoute) {
        if (event.url.pathname.startsWith(e.start)) {
            if (e.excludes.some(elem => event.url.pathname.includes(elem))) {
                break
            }

            if (!user.token) {
                throw redirect(302, '/admin/login')
            }

            if (!e.roles.some(role => checkRole(user.roles ?? [], role))) {
                throw redirect(302, '/error/permission_denied')
            }
        }
    }

    return await resolve(event)
}

// check the role is included by given roles.
//
// return true, if exist
// return false, if not exist
export const checkRole = (roles: string[], role: string) => {
    return roles.some((e) => e.toLowerCase() === role)
}
