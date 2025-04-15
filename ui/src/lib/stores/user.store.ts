import { get, writable } from 'svelte/store'
import Cookies from 'js-cookie'

const createWritableStore = (key: string, initialValue: any) => {
    const { subscribe, set } = writable(initialValue)

    return {
        subscribe,
        set,
        useLocalStorage: () => {
            const u = Cookies.get(key)
            if (u && u !== '') {
                set(JSON.parse(u))
            }

            subscribe((current) => {
                Cookies.set(key, JSON.stringify(current), {
                    expires: 14,
                    path: '/'
                })
            })
        },
        remove: () => {
            user.set(null)
            Cookies.remove(key)
        },
        get: () => {
            return get(user)
        }
    }
}

export const user = createWritableStore('user', null)
