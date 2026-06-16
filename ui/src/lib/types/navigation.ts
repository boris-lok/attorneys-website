type NavItem = {
    icon: string
    name: string
}

type Link = NavItem & { url: string }
type IconButton = NavItem & { onClick: () => void }

export type NavigationItem = Link | IconButton
