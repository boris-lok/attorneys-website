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

export function dateRangeFormatter(startedAt: Date, endedAt: Date): string {
    const baseDateOpts: Intl.DateTimeFormatOptions = {
        year: '2-digit',
        month: '2-digit',
        day: '2-digit',
    }

    const timeOpts: Intl.DateTimeFormatOptions = {
        hour: '2-digit',
        minute: '2-digit',
        hour12: false,
    }

    const sameDay =
        startedAt.getFullYear() === endedAt.getFullYear() &&
        startedAt.getMonth() === endedAt.getMonth() &&
        startedAt.getDate() === endedAt.getDate()

    const st = startedAt.toLocaleString('en-US', {
        ...baseDateOpts,
        ...timeOpts,
    })

    const ed = sameDay
        ? endedAt.toLocaleTimeString('en-US', timeOpts)
        : endedAt.toLocaleString('en-US', {
              ...baseDateOpts,
              ...timeOpts,
          })

    return `${st} ~ ${ed}`
}
