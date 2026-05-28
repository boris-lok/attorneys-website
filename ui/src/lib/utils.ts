import Cookies from 'js-cookie'
import { jwtDecode } from 'jwt-decode'

export type PayLoad = {
    sub: string
    exp: number
    roles: string[]
    nickname: string
}

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms))
}

export function getToken() {
    const token = Cookies.get()
    console.log('token', token)
    if (!token) {
        return ''
    }

    return `Bearer ${token}`
}

function getPayLoad() {
    const token = Cookies.get()['token']
    const payLoad = jwtDecode<PayLoad>(token)
    if (!payLoad) {
        return null
    }
    return payLoad
}

export function getSelfId() {
    return getPayLoad()?.sub ?? ''
}

export function getSelfName() {
    return getPayLoad()?.nickname ?? ''
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
        day: '2-digit'
    }

    const timeOpts: Intl.DateTimeFormatOptions = {
        hour: '2-digit',
        minute: '2-digit',
        hour12: false
    }

    const sameDay =
        startedAt.getFullYear() === endedAt.getFullYear() &&
        startedAt.getMonth() === endedAt.getMonth() &&
        startedAt.getDate() === endedAt.getDate()

    const st = startedAt.toLocaleString('en-US', {
        ...baseDateOpts,
        ...timeOpts
    })

    const ed = sameDay
        ? endedAt.toLocaleTimeString('en-US', timeOpts)
        : endedAt.toLocaleString('en-US', {
            ...baseDateOpts,
            ...timeOpts
        })

    return `${st} ~ ${ed}`
}
