<script lang="ts">
    import type { WorkLog } from '$lib/services/work_log.service'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { formatDateTime } from '$lib/utils'

    type Props = {
        log: WorkLog
        caseId: string
    }

    let { log, caseId }: Props = $props()
    let isEditMode = $state(false)
    let _log = $state(log)

    function onSaved(newValue: WorkLog) {
        _log = newValue
        isEditMode = false
    }


</script>

<div class="flex flex-col shadow my-4 mx-4 rounded">
    {#if isEditMode}
        <div class="p-4">
            <WorkLogEditor
                id={log.id}
                caseId={caseId}
                date={log.startedAt}
                description={log.description}
                duration={log.duration}
                collaboratorIds={log.collaborators.map(collaborator => collaborator.id)}
                onClosed={() => (isEditMode = false)}
                hideShare={true}
                onSaved={onSaved}
            />
        </div>

    {:else}
        <div
            class="mt-2 flex min-h-16 w-full flex-row items-center justify-between gap-2 px-4"
        >
            <div class="flex flex-col w-full">
                <div class="flex flex-row gap-2 w-full justify-between my-2 px-2">
                    <p class="text-sm">{ formatDateTime(_log.startedAt) }</p>
                    <p class="text-sm text-gray-500">{ _log.duration } mins</p>
                </div>

                <div class="w-full h-[1px] bg-gray-200">&nbsp;</div>

                <p class="px-2 my-2 text-md">{_log.description}</p>


                {#if log.collaborators.length > 0}
                    <div class="w-full h-[1px] bg-gray-200">&nbsp;</div>

                    <div class="w-full flex flex-row-reverse flex-wrap">
                        {#each _log.collaborators as collaborator (collaborator.id)}
                            <p class="px-2 my-2 text-sm">{collaborator.name}</p>
                        {/each}
                    </div>
                {/if}
            </div>

            {#if !_log.isCollaborative}
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
            {/if}
        </div>
    {/if}
</div>