import { fromFetch } from 'rxjs/fetch'
import { ADMIN_URL, TIMEOUT } from '$lib/constant'
import { catchError, map, Observable, of } from 'rxjs'
import { getToken } from '$lib/utils'

type Error = { error: boolean; message: unknown | string }
type LoginSuccessResponse = {
    error: boolean
    data: {
        userId: string
        username: string
        token: string
    }
}

function login(req: {
    username: string
    password: string
}): Observable<Error | LoginSuccessResponse> {
    return fromFetch(`${ADMIN_URL}/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(req),
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) => {
            if (!resp.ok) {
                return of({ error: true, message: `Error: ${resp.status}` })
            }
            return resp.json()
        },
    }).pipe(
        map((resp) => {
            if ('token' in resp && 'user_id' in resp && 'username' in resp) {
                return {
                    error: false,
                    data: {
                        userId: resp.user_id,
                        username: resp.username,
                        token: resp.token,
                    },
                }
            } else if ('error' in resp && resp.error === true) {
                return resp
            } else {
                return { error: true, message: `Unknown error: ${resp}` }
            }
        }),
        catchError((err: unknown) => {
            return of({ error: true, message: err })
        }),
    )
}

function logout() {
    return fromFetch(`${ADMIN_URL}/logout`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            Authorization: getToken(),
        },
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) => {
            if (!resp.ok) {
                return of({ error: true, message: `Error: ${resp.status}` })
            }
            return of({ error: false, message: 'logout successful' })
        },
    }).pipe(
        catchError((err: unknown) => {
            return of({ error: true, message: err })
        }),
    )
}

export const UserService = {
    login: login,
    logout: logout,
}
