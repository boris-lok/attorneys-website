import { ADMIN_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

type Error = { error: true; message: string }
type LoginSuccessResponse = {
    error: false
    data: {
        userId: string
        username: string
        token: string
    }
}
type LogoutSuccessResponse = { error: false }

/**
 * Authenticates a user by sending their credentials to the login endpoint.
 *
 * @param {Object} req - An object containing the user's login credentials.
 * @param {string} req.username - The username of the user.
 * @param {string} req.password - The password of the user.
 * @return {Promise<LoginSuccessResponse | Error>} A promise that resolves to an object containing either
 *         the user's token, user ID, and username if successful, or an error message if not.
 */
async function login(req: {
    username: string
    password: string
}): Promise<LoginSuccessResponse | Error> {
    try {
        const resp = await fetch(`${ADMIN_URL}/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(req),
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }

        const json = await resp.json()
        if ('token' in json && 'user_id' in json && 'username' in json) {
            return {
                error: false,
                data: {
                    userId: json.user_id,
                    username: json.username,
                    token: json.token,
                },
            }
        }

        return { error: true, message: `Unknown error: ${json}` }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Logs out the currently authenticated user by making a POST request
 * to the logout endpoint. This will invalidate the user's active session.
 *
 * @return {Promise<Error | LogoutSuccessResponse>} A promise that resolves to an object indicating
 * the success or failure of the logout operation. On success, returns an object with `error: false`.
 * On failure, returns an object with `error: true` and an appropriate error message.
 */
async function logout(): Promise<Error | LogoutSuccessResponse> {
    try {
        const resp = await fetch(`${ADMIN_URL}/logout`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }

        return { error: false }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * UserService provides methods for handling user authentication operations.
 *
 * It includes functionalities such as logging in and logging out the user.
 */
export const UserService = {
    login: login,
    logout: logout,
}
