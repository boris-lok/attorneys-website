<script lang="ts">
    import IconifyIcon from '@iconify/svelte'
    import CaseEditor from '$lib/components/CaseEditor.svelte'
    import { roundTo } from '$lib/utils'
    import type { CaseData } from '$lib/types'
    import ProgressBar from '$lib/components/ProgressBar.svelte'

    type Props = CaseData & {
        onSaved: (data: CaseData) => void
    }

    let { onSaved, ...rest }: Props = $props()
    let copiedData = $state<CaseData>(rest)

    const hrs = $derived(roundTo(copiedData.estimatedMinutes / 60, 2))
    const usedPercentage = $derived(roundTo(copiedData.usedMinutes * 100 / copiedData.estimatedMinutes, 0))
    const usedHrs = $derived(roundTo(copiedData.usedMinutes / 60, 2))


    function formater(date: Date) {
        return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
    }

    function onEditClicked(e: Event) {
        e.preventDefault()
        e.stopPropagation()
        isEditMode = true
    }

    let isEditMode = $state(false)
</script>

{#if isEditMode}
    <CaseEditor id={copiedData.id} name={copiedData.name} hrs={hrs} startedAt={copiedData.startedAt}
                endedAt={copiedData.endedAt} onSaved={onSaved} onClosed={() => isEditMode = false} />
{:else}
    <a href={`/b/case/${copiedData.id}`}>
        <div
            class="md:flex md:flex-row md:items-center md:gap-4 p-4 md:p-2 md:hover:bg-gray-50 shadow rounded m-4 md:m-0 md:rounded-none md:shadow-none md:min-h-12">

            <div class="font-semibold md:font-medium text-nowrap flex-6/12 my-2 md:my-0">
                {copiedData.name}
            </div>

            <div class="text-sm text-gray-500 md:text-gray-700 flex-2/12 my-1 md:my-0">
                {formater(copiedData.startedAt)} -> {formater(copiedData.endedAt)}
            </div>

            <div class="text-sm md:text-right flex-2/12">
                <div class="md:gap-2 md:items-center flex-col-reverse flex md:flex-row">
                    <div class="w-full md:w-20">
                        <ProgressBar progress={usedPercentage} />
                    </div>
                    <div class="flex flex-row gap-1 my-1">
                        <p>
                            {usedHrs}/{hrs}
                        </p>

                        <p class="md:hidden text-sm text-gray-500">
                            hrs
                        </p>
                        <p>
                            ({usedPercentage}%)
                        </p>
                    </div>

                </div>

            </div>

            <div class="flex h-fit flex-row gap-2 flex-auto justify-end">
                <button
                    class="cursor-pointer mt-4 md:mt-0"
                    onclick={onEditClicked}
                >
                    <IconifyIcon
                        class="h-4 w-4 md:h-6 md:w-6 hidden md:block hover:text-green-400"
                        icon="lucide:edit"
                    />
                    <span class="md:hidden">Edit</span>
                </button>
            </div>
        </div>
    </a>
{/if}
