import type { PageLoad } from './$types'
import { ArticleServices } from '$lib/services/article.service'
import type { Language, SimpleArticle } from '$lib/types'

const PAGE_SIZE = 10

export const load: PageLoad = async ({ fetch, url }) => {
    const lang: Language = 'zh'
    const categoryId = url.searchParams.get('category')
    const page = Number(url.searchParams.get('page') ?? '0')

    const resp = await ArticleServices.list(fetch, lang, categoryId, page, PAGE_SIZE)

    const articles: SimpleArticle[] = resp.error ? [] : (resp.articles ?? [])

    return {
        articles,
        totalPages: resp.error ? 0 : Math.ceil((resp.total ?? 0) / PAGE_SIZE),
        page,
        lang,
        categoryId,
    }
}
