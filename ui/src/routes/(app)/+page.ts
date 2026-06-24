import { HomeServices } from '$lib/services/home.service'
import type { Language } from '$lib/types'
import type { PageLoad } from './$types'
import { error } from '@sveltejs/kit'

export const load: PageLoad = async ({ fetch }) => {
    let lang: Language = 'zh'
    const resp = await HomeServices.list(fetch, lang)

    if (resp.error) {
        throw error(502, resp.message ?? 'Failed to fetch data')
    }

    const home = resp.home[0]
    if (!home) {
        throw error(404, 'Content not found')
    }

    return {
        data: home.data.data,
    }
}
