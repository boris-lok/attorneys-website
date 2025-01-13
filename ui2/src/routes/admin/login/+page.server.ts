import { type Actions, type Cookies, fail, redirect } from '@sveltejs/kit'
import { UserService } from '$lib/services/user.service'
import { lastValueFrom } from 'rxjs'

export const load = ({ cookies }) => {
    const token = cookies.get('token')

    if (token) {
        return redirect(302, '/admin/dashboard')
    }

    return {}
}

const login = async ({
    cookies,
    request,
}: {
    cookies: Cookies
    request: Request
}) => {
    const data = await request.formData()
    const username = data.get('username')
    const password = data.get('password')

    if (!username || !password) {
        console.error('login failed username or password are empty')
        return fail(400, { incorrect: true })
    }

    if (typeof username === 'string' && typeof password === 'string') {
        const resp = await lastValueFrom(
            UserService.login({ username: username, password: password }),
        )
        if (resp.error) {
            console.error(`login failed: ${JSON.stringify(resp)}`)

            return fail(400, { incorrect: true })
        } else if (!resp.error && 'data' in resp) {
            cookies.set('token', resp.data.token, {
                // send cookie for every page
                path: '/',
                // server side only cookie so you can't use `document.cookie`
                httpOnly: true,
                // set cookie to expire after a month
                maxAge: 60 * 60 * 24 * 30,
            })

            redirect(302, '/admin/dashboard')
        }
    } else {
        console.error('login failed username or password are not strings')
        return fail(400, { incorrect: true })
    }
}

export const actions = {
    default: login,
} satisfies Actions
