export type Column = {
    label: string
    class?: string // for any extra per-cell classes (text-nowrap, etc.)
}

export const CASE_COLUMNS: Column[] = [
    { label: 'Case Name' },
    { label: 'Period' },
    { label: 'Used Hrs', class: 'text-nowrap' },
    { label: 'Next Billing', class: 'text-nowrap' },
    { label: 'Last Billing', class: 'text-nowrap' },
]

// Single source of truth for column sizing
export const GRID_COLS_WITH_ACTIONS = 'md:grid-cols-[3fr_2fr_2fr_1fr_1fr_auto]'
export const GRID_COLS_WITHOUT_ACTIONS = 'md:grid-cols-[3fr_2fr_2fr_1fr_1fr]'
