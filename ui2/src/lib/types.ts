type NavItem = {
    icon: string
    name: string
}

type Link = NavItem & { url: string }
type IconButton = NavItem & { onClick: () => void }

export type NavigationItem = Link | IconButton

// The language enum that we support
export type Language = 'en' | 'zh'

// The request of creating home content
export type CreateHomeRequest = {
    data: string
    language: Language
    seq: number
}

// The request of updating home content
export type UpdateHomeRequest = CreateHomeRequest & { id: string }
