import type { Credential } from '$lib/services/user.service'
import { redirect } from '@sveltejs/kit'
import { checkRole } from '../../hooks.server'

export const load = async ({ url, cookies }) => {
    const user: Partial<Credential> = JSON.parse(cookies.get('user') || '{}')

    if (url.pathname === '/admin/login') {
        return
    }

    if (!user.token) {
        throw redirect(302, '/admin/login')
    }

    if (!checkRole(user.roles ?? [], 'admin')) {
        throw redirect(302, '/error/permission_denied')
    }
}
