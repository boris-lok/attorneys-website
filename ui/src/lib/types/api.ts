export type APIError = {
    error: true
    message: string
}

export type APIResponse<T> = T extends void ? { error: false } : { error: false } & T
