import { redirect } from '@sveltejs/kit'
import { checkRole } from '../../hooks.server'

export const load = async ({ locals, url }) => {
    if (url.pathname === '/admin/login') return

    const user = 'user' in locals ? locals.user : null

    if (!user) {
        throw redirect(302, '/admin/login')
    }

    if (!checkRole(user.roles ?? [], 'admin')) {
        throw redirect(302, '/error/permission_denied')
    }
}
