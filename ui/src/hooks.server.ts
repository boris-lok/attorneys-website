import { jwtDecode } from 'jwt-decode'
import type { PayLoad } from '$lib/utils'

export const handle = async ({ event, resolve }) => {
    const token = event.cookies.get('token')

    let payload: PayLoad | null = null
    if (token) {
        try {
            payload = jwtDecode<PayLoad>(token)
            event.locals.user = {
                id: payload.sub,
                roles: payload.roles ?? [],
                nickname: payload.nickname ?? '',
            }
        } catch {
            payload = null
            event.locals.user = null
        }
    }

    return await resolve(event)
}
