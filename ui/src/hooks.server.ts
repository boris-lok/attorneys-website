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
        excludes: ['/admin/login'],
        roles: ['admin'],
    },
    {
        start: '/b',
        excludes: [],
        roles: ['admin', 'lawyer'],
    },
]

export const handle = async ({ event, resolve }) => {
    const token = event.cookies.get('token')

    let payload: PayLoad | null = null
    if (token) {
        try {
            payload = jwtDecode<PayLoad>(token)
        } catch {
            payload = null
        }
    }

    for (const rule of validateRoute) {
        if (!event.url.pathname.startsWith(rule.start)) continue

        const isExcluded = rule.excludes.some(
            (path) => event.url.pathname === path || event.url.pathname.startsWith(`${path}/`)
        )
        if (isExcluded) continue

        if (!payload) {
            event.locals.user = null
            throw redirect(302, '/admin/login')
        }

        const hasRole = rule.roles.some((role) => checkRole(payload.roles ?? [], role))
        if (!hasRole) {
            throw redirect(302, '/error/permission_denied')
        }
    }

    if (payload) {
        event.locals.user = {
            id: payload.sub,
            roles: payload.roles ?? [],
        }
    }

    return await resolve(event)
}

// check if `role` is included in `roles` (case-insensitive)
export const checkRole = (roles: string[], role: string) => {
    const target = role.toLowerCase()
    return roles.some((e) => e.toLowerCase() === target)
}
