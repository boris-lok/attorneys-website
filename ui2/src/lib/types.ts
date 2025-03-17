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

// The data structure of home data
export type HomeData = {
    id: string
    language: Language
    data: {
        data: string
    }
}

// The data structure of service data
export type ServiceData = {
    id: string
    language: Language
    data: {
        title: string
        data: string
    }
    seq: number
}

// The request of creating service content
export type CreateServiceRequest = {
    title: string
    data: string
    language: Language
    seq: number
}

// The request of updating service content
export type UpdateServiceRequest = { id: string } & CreateServiceRequest
