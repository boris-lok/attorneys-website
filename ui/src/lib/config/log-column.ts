import { CASE_GRID_COLS_WITHOUT_ACTIONS } from '$lib/config/column'

export type Column = {
    label: string
    class?: string
}

export const PENDING_LOG_COLUMNS: Column[] = [
    { label: 'Description' },
    { label: 'Period' },
    { label: 'Used Hrs' },
    { label: '\u00a0' }, // &nbsp;
]

export const WORK_LOG_COLUMNS: Column[] = [
    { label: 'Description' },
    { label: 'Period' },
    { label: 'Used Hrs' },
    { label: 'Participants' },
    { label: '\u00a0' }, // &nbsp;
]

export const CLOSED_WORK_LOG_COLUMNS: Column[] = [
    { label: 'Description' },
    { label: 'Period' },
    { label: 'Used Hrs' },
    { label: 'Participants' },
]

export const PENDING_LOG_GRID_COLS = 'md:grid-cols-[5fr_4fr_1fr_2fr]'
export const WORK_LOG_GRID_COLS_WITH_ACTIONS = 'md:grid-cols-[5fr_2fr_1fr_2fr_1fr]'
export const WORK_LOG_GRID_COLS_WITHOUT_ACTIONS = 'md:grid-cols-[5fr_2fr_1fr_2fr]'
