import type { PageServerLoad } from './$types'

export const load: PageServerLoad = ({ params, locals }) => {
    return {
        caseId: params.slug,
        user: locals.user,
    }
}
