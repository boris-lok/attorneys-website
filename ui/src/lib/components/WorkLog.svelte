<script lang="ts">
    import type { WorkLog } from '$lib/services/work_log.service'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { roundTo } from '$lib/utils'

    type Props = {
        log: WorkLog
        caseId: string
        onSaved: (data: WorkLog) => void
    }

    let { log, caseId }: Props = $props()
    let isEditMode = $state(false)
    let copiedData = $state(log)

    function onSaved(newValue: WorkLog) {
        copiedData = newValue
        isEditMode = false
    }

    const hrs = $derived(roundTo(copiedData.duration / 60, 2))

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
</script>

{#if isEditMode}
    <div class="p-4">
        <WorkLogEditor
            id={log.id}
            {caseId}
            startedAt={log.startedAt}
            endedAt={log.endedAt}
            description={log.description}
            collaboratorIds={log.collaborators.map(
                (collaborator) => collaborator.userId,
            )}
            onClosed={() => (isEditMode = false)}
            hideShare={true}
            {onSaved}
        />
    </div>
{:else}
    <div
        class="m-4 rounded p-4 shadow md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:rounded-none md:p-2 md:shadow-none md:hover:bg-gray-50"
    >
        <div
            class="my-2 flex-5/12 font-semibold text-nowrap md:my-0 md:font-medium"
        >
            {copiedData.description}
        </div>

        <div
            class="my-1 flex-2/12 text-sm text-gray-500 md:my-0 md:text-gray-700"
        >
            {formater(copiedData.startedAt)}
        </div>

        <div class="flex-1/12 text-sm">
            <span class="text-gray-600 md:hidden">Duration: </span>
            <span>{hrs} hrs</span>
        </div>

        <div class="flex-2/12 text-sm">
            <span class="text-gray-600 md:hidden">Participants: </span>
            <span>{copiedData.user.name}</span>
            {#if copiedData.collaborators.length > 0}
                {#each copiedData.collaborators as collaborator (collaborator.userId)}
                    {#if collaborator.status !== 'rejected'}
                        <span>, </span>
                        <span
                            class:text-amber-600={collaborator.status ===
                                'pending'}
                        >
                            {collaborator.name}
                        </span>
                    {/if}
                {/each}
            {/if}
        </div>

        {#if !copiedData.isCollaborative}
            <div class="flex h-fit flex-1/12 flex-row justify-end gap-2">
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
            </div>
        {:else}
            <div class="flex h-fit flex-1/12 flex-row justify-end gap-2">
                &nbsp;
            </div>
        {/if}
    </div>
{/if}
