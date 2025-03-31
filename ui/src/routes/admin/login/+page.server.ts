import { redirect } from '@sveltejs/kit'

export const load = ({ cookies }) => {
    const token = cookies.get('token')

    if (token) {
        return redirect(302, '/admin/dashboard')
    }

    return {}
}
