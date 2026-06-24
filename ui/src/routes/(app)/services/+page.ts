import { ServiceServices } from '$lib/services/service.service'
import type { Language } from '$lib/types'
import { error } from '@sveltejs/kit'
import type { PageLoad } from './$types'

export const load: PageLoad = async () => {
    const lang: Language = 'zh'
    const resp = await ServiceServices.list(lang)

    if (resp.error) {
        throw error(502, resp.message ?? '')
    }

    const services = resp.services

    return {
        data: services,
    }
}
