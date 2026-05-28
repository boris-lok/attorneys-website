import type { PageServerLoad } from './$types'

export const load: PageServerLoad = ({ params, cookies }) => {
    const token = cookies.get('token')

    return {
        id: params.slug,
        token
    }
}