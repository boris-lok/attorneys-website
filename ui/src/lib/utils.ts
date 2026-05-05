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

export function getSelfId() {
    const u = user.get()
    if (!u) {
        return ''
    }
    return u.userId
}

export function getSelfName() {
    const u = user.get()
    if (!u) {
        return ''
    }
    return u.nickname
}

export function formatDateTime(date: Date): string {
    const pad = (n: number) => String(n).padStart(2, '0')
    const y = date.getFullYear()
    const m = pad(date.getMonth() + 1)
    const d = pad(date.getDate())
    const hh = pad(date.getHours())
    const mm = pad(date.getMinutes())
    return `${y}/${m}/${d} ${hh}:${mm}`
}

export function roundTo(num: number, decimals: number): number {
    return Math.round(num * Math.pow(10, decimals)) / Math.pow(10, decimals)
}
