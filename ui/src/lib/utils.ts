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

export function roundTo(num: number, decimals: number): number {
    return Math.round(num * Math.pow(10, decimals)) / Math.pow(10, decimals)
}
