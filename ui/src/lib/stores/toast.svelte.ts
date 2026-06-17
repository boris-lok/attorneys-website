type ToastSvelte = {
    id: string
    message: string
    type: 'success' | 'error'
}

const createToastStore = () => {
    let toasts = $state<ToastSvelte[]>([])

    function show(message: string, type: ToastSvelte['type'] = 'success', duration: number = 3000) {
        const id = crypto.randomUUID()
        toasts.push({ id, message, type })

        setTimeout(() => dismiss(id), duration)
    }

    function dismiss(id: string) {
        // Here, we don't assign the new array,
        // because $state in ts file (not svelte.ts) only track the mutation
        //
        // Initial: toasts → [Proxy] ✅ Svelte watches this
        // After filter: toasts → [new plain array] ❌ Svelte no longer watching
        const index = toasts.findIndex((toast) => toast.id === id)
        if (index !== -1) toasts.splice(index, 1)
    }

    return {
        get toasts() {
            return toasts
        },
        show,
        dismiss,
    }
}

export const toast = createToastStore()
