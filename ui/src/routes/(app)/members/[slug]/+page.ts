import type { PageLoad } from './$types'
import { MemberServices } from '$lib/services/member.service'
import { error } from '@sveltejs/kit'

export const load: PageLoad = async ({ params }) => {
    const resp = await MemberServices.retrieve(params.slug, 'zh')

    if (resp.error) {
        throw error(502, resp.message)
    }

    return {
        data: resp.member,
    }
}
