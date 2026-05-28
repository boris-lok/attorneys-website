import { redirect } from '@sveltejs/kit'
import { jwtDecode } from 'jwt-decode'
import type { PayLoad } from '$lib/utils'

type ValidateRoute = {
    start: string
    excludes: string[]
    roles: string[]
}

const validateRoute: ValidateRoute[] = [
    {
        start: '/admin',
        excludes: ['login'],
        roles: ['admin']
    },
    {
        start: '/b',
        excludes: [],
        roles: ['admin', 'lawyer']
    }
]

export const handle = async ({ event, resolve }) => {
    const token = event.cookies.get('token')

    let payLoad: PayLoad | null = null
    try {
        payLoad = jwtDecode<PayLoad>(token ?? '')
    } catch (e) {
        payLoad = null
    }

    for (const e of validateRoute) {
        if (event.url.pathname.startsWith(e.start)) {
            if (e.excludes.some((elem) => event.url.pathname.includes(elem))) {
                break
            }

            if (!payLoad || !token) {
                event.locals.user = null
                throw redirect(302, '/admin/login')
            }

            if (!e.roles.some((role) => checkRole(payLoad.roles ?? [], role))) {
                throw redirect(302, '/error/permission_denied')
            }
        }
    }

    if (payLoad) {
        event.locals.user = {
            id: payLoad.sub,
            roles: payLoad.roles ?? []
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
