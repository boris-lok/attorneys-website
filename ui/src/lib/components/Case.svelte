<script lang="ts">
    import IconifyIcon from '@iconify/svelte'
    import CaseEditor from '$lib/components/CaseEditor.svelte'
    import { roundTo } from '$lib/utils'
    import type { CaseData } from '$lib/types'

    type Props = {
        id: string
        name: string
        estimatedMinutes: number
    }

    let { id, name, estimatedMinutes }: Props = $props()

    function onSaved(c: CaseData) {
        id = c.id
        name = c.name
        estimatedMinutes = c.estimatedMinutes
    }

    const hrs = $derived(roundTo(estimatedMinutes / 60, 1))

    let isEditMode = $state(false)
</script>

<div class="flex flex-col">
    {#if isEditMode}
        <CaseEditor
            {id}
            {name}
            hrs={roundTo(estimatedMinutes / 60, 1)}
            onClosed={() => (isEditMode = false)}
            {onSaved}
        />
    {:else}
        <div
            class="mt-2 flex h-16 w-full flex-row items-center justify-between gap-2 px-4"
        >
            <p class="flex-4/6 text-xl text-black md:text-2xl">{name}</p>
            <p class="md:text-md h-fit flex-1/6 text-sm text-gray-500">
                {hrs} hrs
            </p>

            <div class="flex h-fit flex-row gap-2">
                <button
                    class="cursor-pointer"
                    onclick={() => (isEditMode = true)}
                >
                    <IconifyIcon
                        class="h-4 w-4 md:h-6 md:w-6"
                        icon="lucide:edit"
                    />
                </button>
            </div>
        </div>
    {/if}
</div>
