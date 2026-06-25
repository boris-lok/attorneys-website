<script lang="ts">

    import { confirmState, resolveConfirm } from '$lib/composables/confirm.svelte'

    function onKeydown(e: KeyboardEvent) {
        if (e.key === 'Escape') resolveConfirm(false)
    }
</script>

<svelte:window onkeydown={confirmState.open ? onKeydown : undefined} />

{#if confirmState.open}
    <div
        class="fixed inset-0 z-50 grid place-items-center bg-black/50 p-4"
        role="presentation"
        onclick={() => resolveConfirm(false)}
    >
        <!-- svelte-ignore a11y_interactive_supports_focus -->
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div
            class="w-full max-w-md rounded-xl bg-white p-6 shadow-2xl"
            role="alertdialog"
            aria-modal="true"
            aria-labelledby="confirm-title"
            onclick={(e) => e.stopPropagation()}
        >
            {#if confirmState.options.title}
                <h2 id="confirm-title" class="mb-2 text-lg font-semibold text-gray-900">
                    {confirmState.options.title}
                </h2>
            {/if}

            <p class="mb-6 text-gray-600">
                {confirmState.options.message}
            </p>

            <div class="flex justify-end gap-3">
                <button
                    class="rounded-lg border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 cursor-pointer"
                    onclick={() => resolveConfirm(false)}
                >
                    {confirmState.options.cancelText ?? 'Cancel'}
                </button>
                <button
                    class="rounded-lg px-4 py-2 text-sm font-medium text-white {confirmState
            .options.danger
            ? 'bg-red-500 hover:bg-red-600'
            : 'bg-blue-500 hover:bg-blue-600'} cursor-pointer"
                    onclick={() => resolveConfirm(true)}
                >
                    {confirmState.options.confirmText ?? 'Confirm'}
                </button>
            </div>
        </div>
    </div>
{/if}