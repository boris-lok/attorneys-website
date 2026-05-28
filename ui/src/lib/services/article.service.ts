import type {
    APIError,
    APIResponse,
    ArticleData,
    CreateArticleRequest,
    Language,
    UpdateArticleRequest,
} from '$lib/types'
import { ADMIN_URL, BASE_URL, TIMEOUT } from '$lib/constant'
import { getToken } from '$lib/utils'

/**
 * Saves an article by sending a request to the server.
 * It determines whether to create or update an article based on the presence of the `id` field in the request.
 *
 * @param {CreateArticleRequest | UpdateArticleRequest} req The request object containing article data.
 * If it includes an `id`, the article will be updated; otherwise, a new article will be created.
 *
 * @return {Promise<APIError | APIResponse<void>> A promise that resolves to an object indicating success or failure.
 * The `error` field is true if an error occurs, and `message` provides details about the error when applicable.
 */
async function save(
    req: CreateArticleRequest | UpdateArticleRequest
): Promise<APIError | APIResponse<void>> {
    try {
        const resp = await fetch(`${ADMIN_URL}/articles`, {
            method: 'id' in req ? 'PUT' : 'POST',
            headers: {
                'Content-Type': 'application/json',
                Authorization: getToken(),
            },
            body: JSON.stringify(req),
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }
        return { error: false }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Fetches an article from the specified API endpoint based on the given ID and language.
 *
 * @param {string} id - The unique identifier of the article to retrieve.
 * @param {Language} language - The language in which the article should be fetched.
 * @return {Promise<{error: boolean, article?: ArticleData, message?: string}>} A promise that resolves with the response object.
 *         If successful, the object contains the article data.
 *         If an error occurs, the object contains an error flag and a message describing the issue.
 */
async function retrieve(
    id: string,
    language: Language
): Promise<APIError | APIResponse<{ article: ArticleData }>> {
    try {
        const resp = await fetch(`${BASE_URL}/articles/${id}`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Accept-Language': language,
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        if (!resp.ok) {
            return { error: true, message: `Error: ${resp.status}` }
        }

        const json = await resp.json()
        if ('article' in json) {
            return { error: false, article: json.article as ArticleData }
        } else {
            return { error: true, message: `Error: can't decode json: ${json}` }
        }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

/**
 * Fetches a list of articles based on the specified language, category, pagination, and page size.
 *
 * @param {Language} language - The language in which the articles should be retrieved.
 * @param {string | null} categoryId - The ID of the category to filter articles by, or null if no category filter is needed.
 * @param {number} page - The page number to retrieve.
 * @param {number} pageSize - The number of articles to retrieve per page.
 * @return {Promise<APIError | APIResponse<{articles: {id: string, title: string, language: Language, createdAt: Date, createdAtString: string, seq: number}, total: number}>>}
 *         A promise that resolves to an object containing either the list of articles and total count if successful, or an error message if an error occurs.
 */
async function list(
    language: Language,
    categoryId: string | null,
    page: number,
    pageSize: number
): Promise<
    | APIError
    | APIResponse<{
          articles: {
              id: string
              title: string
              language: Language
              createdAt: Date
              createdAtString: string
              seq: number
          }
          total: number
      }>
> {
    let url = `${BASE_URL}/articles?page=${page}&page_size=${pageSize}`
    if (categoryId) {
        url += `&category_id=${categoryId}`
    }

    try {
        const resp = await fetch(url, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
                'Accept-Language': language,
            },
            signal: AbortSignal.timeout(TIMEOUT),
        })

        const json = await resp.json()
        let articles = []

        if ('articles' in json && json.articles.length > 0) {
            articles = json.articles.map(
                (article: {
                    id: string
                    created_at: string
                    title: string
                    language: Language
                    seq: number
                }) => {
                    const date = new Date(article.created_at)

                    return {
                        id: article.id,
                        title: article.title,
                        language: article.language,
                        createdAt: date,
                        createdAtString: `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`,
                        seq: article.seq,
                    }
                }
            )
        }

        return { error: false, articles: articles, total: json.total }
    } catch (error) {
        return { error: true, message: `Error: ${error}` }
    }
}

export const ArticleServices = {
    save: save,
    retrieve: retrieve,
    list: list,
}
