import type { ArticleData, CreateArticleRequest, Language, UpdateArticleRequest } from '$lib/types'
import { fromFetch } from 'rxjs/fetch'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'
import { map } from 'rxjs'

/**
 * The API endpoint of saving article
 * @param req
 */
function save(req: CreateArticleRequest | UpdateArticleRequest) {
    let method = 'id' in req ? 'PUT' : 'POST'

    return fromFetch(`${ADMIN_URL}/articles`, {
        method: method,
        headers: {
            'Content-Type': 'application/json',
            Authorization: getToken(),
        },
        body: JSON.stringify(req),
        signal: AbortSignal.timeout(TIMEOUT),
    }).pipe(
        map((resp) => {
            if (!resp.ok) {
                return { error: true, message: `Error: ${resp.status}` }
            }
            return { error: false }
        }),
    )
}

/**
 * The API endpoint of retrieving the article by id and language
 * @param id the ID of article
 * @param language the language
 */
function retrieve(id: string, language: Language) {
    return fromFetch(`${BASE_URL}/articles/${id}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Accept-Language': language,
        },
        signal: AbortSignal.timeout(TIMEOUT),
        selector: (resp) =>
            resp.json().then((json) => {
                return 'article' in json ? (json.article as ArticleData) : null
            }),
    })
}

function list(language: Language, page: number, pageSize: number) {
    return fromFetch(
        `${BASE_URL}/articles?page=${page}&page_size=${pageSize}`,
        {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Accept-Language': language,
            },
            signal: AbortSignal.timeout(TIMEOUT),
            selector: (resp) =>
                resp.json().then((value) => {
                    const articles = []

                    if ('articles' in value && value.articles.length > 0) {
                        for (const article of value.articles) {
                            const date = new Date(article.created_at)
                            articles.push({
                                id: article.id,
                                title: article.title,
                                language: article.language,
                                createdAt: date,
                                createdAtString: `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`,
                                seq: article.seq,
                            })
                        }
                    }

                    return {
                        articles: articles,
                        total: value.total,
                    }
                }),
        },
    )
}

export const ArticleServices = {
    // save the content of article page.
    save: save,
    // retrieve the article
    retrieve: retrieve,
    // list the articles by page and page size
    list: list,
}
