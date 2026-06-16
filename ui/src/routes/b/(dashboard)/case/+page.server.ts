import type { PageServerLoad } from './$types'
import { CaseServices } from '$lib/services/case.service'
import { error } from '@sveltejs/kit'

export const load: PageServerLoad = async () => {
    const resp = await CaseServices.list()

    if (resp.error) {
        error(500, resp.message)
    }

    return { cases: resp.cases }
}
