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
        return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' })
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
            caseId={caseId}
            date={log.startedAt}
            description={log.description}
            duration={log.duration}
            collaboratorIds={log.collaborators.map(collaborator => collaborator.userId)}
            onClosed={() => (isEditMode = false)}
            hideShare={true}
            onSaved={onSaved}
        />
    </div>

{:else}
    <div
        class="md:flex md:flex-row md:items-center md:gap-4 p-4 md:p-2 md:hover:bg-gray-50 shadow rounded m-4 md:m-0 md:rounded-none md:shadow-none md:min-h-12">
        <div class="font-semibold md:font-medium text-nowrap flex-5/12 my-2 md:my-0">
            {copiedData.description}
        </div>

        <div class="text-sm text-gray-500 md:text-gray-700 flex-2/12 my-1 md:my-0">
            {formater(copiedData.startedAt)}
        </div>

        <div class="text-sm flex-1/12">
            <span class="md:hidden text-gray-600">Duration: </span>
            <span>{hrs} hrs</span>
        </div>

        <div class="text-sm flex-2/12">
            <span class="md:hidden text-gray-600">Participants: </span>
            <span>{copiedData.user.name}</span>
            {#if copiedData.collaborators.length > 0}
                {#each copiedData.collaborators as collaborator (collaborator.userId)}
                    {#if collaborator.status !== 'rejected'}
                        <span>, </span>
                        <span
                            class:text-amber-600={collaborator.status === 'pending'}
                        >
                        {collaborator.name}
                    </span>
                    {/if}
                {/each}
            {/if}
        </div>

        {#if !copiedData.isCollaborative}
            <div class="flex h-fit flex-row gap-2 flex-1/12 justify-end">
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
        {:else}
            <div class="flex h-fit flex-row gap-2 flex-1/12 justify-end">&nbsp;</div>
        {/if}
    </div>
{/if}