import { user } from '$lib/stores/user.store'

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms))
}

export function getToken() {
    const u = user.get()
    if (!u) {
        return ''
    }

    return `Bearer ${u.token}`
}
