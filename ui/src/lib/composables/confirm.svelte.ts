type ConfirmOptions = {
    title?: string
    message: string
    confirmText?: string
    cancelText?: string
    danger?: boolean
}

let resolveFn: ((value: boolean) => void) | null = null

export const confirmState = $state<{
    open: boolean
    options: ConfirmOptions
}>({
    open: false,
    options: { message: '' },
})

export function confirm(options: ConfirmOptions): Promise<boolean> {
    confirmState.options = options
    confirmState.open = true
    return new Promise((resolve) => {
        resolveFn = resolve
    })
}

export function resolveConfirm(value: boolean) {
    confirmState.open = false
    resolveFn?.(value)
    resolveFn = null
}
