import { redirect } from '@sveltejs/kit'
import { checkRole } from '../../hooks.server'

export const load = async ({ locals, url }) => {
    if (url.pathname === '/admin/login') return

    if (!locals.user) {
        throw redirect(302, '/admin/login')
    }

    if (!checkRole(locals.user.roles ?? [], 'admin') && !checkRole(locals.user.roles ?? [], 'lawyer')) {
        throw redirect(302, '/error/permission_denied')
    }
}
