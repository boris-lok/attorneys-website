export type Language = 'en' | 'zh'

export type BaseData<T> = {
    id: string
    language: Language
    data: T
}

export type WithId<T> = T & { id: string }
