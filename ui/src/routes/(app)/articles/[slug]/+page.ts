import type { PageLoad } from './$types'
import type { Language } from '$lib/types'
import { ArticleServices } from '$lib/services/article.service'
import { error } from '@sveltejs/kit'

export const load: PageLoad = async ({ fetch, params }) => {
    const lang: Language = 'zh'

    const resp = await ArticleServices.retrieve(fetch, params.slug, lang)

    if (resp.error) {
        throw error(502, resp.message)
    }

    return {
        article: resp.article,
    }
}
