<script lang="ts">
    import IconifyIcon from '@iconify/svelte'
    import CaseEditor from '$lib/components/CaseEditor.svelte'
    import { roundTo } from '$lib/utils'
    import type { CaseData } from '$lib/types'
    import ProgressBar from '$lib/components/ProgressBar.svelte'
    import { CaseServices } from '$lib/services/case.service'

    type Props = CaseData & {
        onSaved: (data: CaseData) => void
        onDeleted: () => void
    }

    let { onSaved, onDeleted, ...rest }: Props = $props()
    let copiedData = $state<CaseData>(rest)

    const hrs = $derived(roundTo(copiedData.estimatedMinutes / 60, 2))
    const usedPercentage = $derived(
        roundTo(
            (copiedData.usedMinutes * 100) / copiedData.estimatedMinutes,
            0,
        ),
    )
    const usedHrs = $derived(roundTo(copiedData.usedMinutes / 60, 2))

    function formater(date: Date) {
        return date.toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'short',
            day: 'numeric',
        })
    }

    function onEditClicked(e: Event) {
        e.preventDefault()
        e.stopPropagation()
        isEditMode = true
    }

    async function onDeleteClicked(e: Event) {
        e.preventDefault()
        e.stopPropagation()

        const confirmed = confirm(
            'Are you sure you want to delete this case? This action cannot be undone.',
        )

        if (confirmed) {
            const resp = await CaseServices.delete(copiedData.id)
            if (resp.error) {
                alert(resp.message)
                return
            }

            alert('Case deleted successfully')
            onDeleted()
        }
    }

    function _onSaved(data: CaseData) {
        data.usedMinutes = copiedData.usedMinutes
        onSaved(data)
    }

    let isEditMode = $state(false)
</script>

{#if isEditMode}
    <CaseEditor
        id={copiedData.id}
        name={copiedData.name}
        {hrs}
        startedAt={copiedData.startedAt}
        endedAt={copiedData.endedAt}
        onSaved={_onSaved}
        onClosed={() => (isEditMode = false)}
    />
{:else}
    <a href={`/b/case/${copiedData.id}`}>
        <div
            class="m-4 rounded p-4 shadow md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:rounded-none md:p-2 md:shadow-none md:hover:bg-gray-50"
        >
            <div
                class="my-2 flex-6/12 font-semibold text-nowrap md:my-0 md:font-medium"
            >
                {#if copiedData.pendingLogs > 0}
                    <div
                        class="inline-block h-4 w-4 rounded-[50%] border bg-red-500 text-center text-sm"
                    >
                        <p class="text-xs text-white">
                            {copiedData.pendingLogs}
                        </p>
                    </div>
                {/if}
                {copiedData.name}
            </div>

            <div
                class="my-1 flex-2/12 text-sm text-gray-500 md:my-0 md:text-gray-700"
            >
                {formater(copiedData.startedAt)} -> {formater(
                    copiedData.endedAt,
                )}
            </div>

            <div class="flex-2/12 text-sm md:text-right">
                <div
                    class="flex flex-col-reverse md:flex-row md:items-center md:gap-2"
                >
                    <div class="w-full md:w-20">
                        <ProgressBar progress={usedPercentage} />
                    </div>
                    <div class="my-1 flex flex-row gap-1">
                        <p>
                            {usedHrs}/{hrs}
                        </p>

                        <p class="text-sm text-gray-500 md:hidden">hrs</p>
                        <p>
                            ({usedPercentage}%)
                        </p>
                    </div>
                </div>
            </div>

            <div class="flex h-fit flex-auto flex-row justify-end gap-2">
                <button
                    class="mt-4 cursor-pointer md:mt-0"
                    onclick={onEditClicked}
                >
                    <IconifyIcon
                        class="hidden h-4 w-4 hover:text-green-400 md:block md:h-6 md:w-6"
                        icon="lucide:edit"
                    />
                    <span class="md:hidden">Edit</span>
                </button>

                <button
                    class="mt-4 cursor-pointer md:mt-0"
                    onclick={onDeleteClicked}
                >
                    <IconifyIcon
                        class="hidden h-4 w-4 hover:text-red-500 md:block md:h-6 md:w-6"
                        icon="mi:delete"
                    />
                    <span class="md:hidden">Delete</span>
                </button>
            </div>
        </div>
    </a>
{/if}
