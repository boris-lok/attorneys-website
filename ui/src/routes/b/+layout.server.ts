import type { Credential } from '$lib/services/user.service'
import { redirect } from '@sveltejs/kit'
import { checkRole } from '../../hooks.server'

export const load = async ({ cookies }) => {
    const user: Partial<Credential> = JSON.parse(cookies.get('user') || '{}')

    if (!user.token) {
        throw redirect(302, '/admin/login')
    }

    if (
        !checkRole(user.roles ?? [], 'admin') &&
        !checkRole(user.roles ?? [], 'lawyer')
    ) {
        throw redirect(302, '/error/permission_denied')
    }
}
