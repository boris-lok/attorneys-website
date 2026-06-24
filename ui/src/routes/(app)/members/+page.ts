import type { PageLoad } from './$types'
import { MemberServices } from '$lib/services/member.service'
import type { Language } from '$lib/types'
import { error } from '@sveltejs/kit'

export const load: PageLoad = async ({ fetch }) => {
    const lang: Language = 'zh'
    const resp = await MemberServices.list(fetch, lang)

    if (resp.error) {
        throw error(502, resp.message)
    }

    const members = resp.members

    return { data: members }
}
