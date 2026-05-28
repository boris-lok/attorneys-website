import type { PageLoad } from './$types'
import { CaseServices } from '$lib/services/case.service'

export const load: PageLoad = () => {
    return {
        cases: CaseServices.list(),
    }
}
