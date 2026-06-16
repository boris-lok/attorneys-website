import type { PageServerLoad } from './$types'
import { createCaseServices } from '$lib/services/case.service'
import { error } from '@sveltejs/kit'

export const load: PageServerLoad = async ({ fetch }) => {
    const s = createCaseServices(fetch)
    const resp = await s.list()

    if (resp.error) {
        error(500, resp.message)
    }

    return { cases: resp.cases }
}
