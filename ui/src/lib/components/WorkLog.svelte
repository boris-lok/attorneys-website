<script lang="ts">
    import {
        type WorkLog,
        WorkLogServices,
    } from '$lib/services/work_log.service'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { getSelfId, roundTo } from '$lib/utils'
    import { CaseServices } from '$lib/services/case.service'

    type Props = {
        log: WorkLog
        caseId: string
        onSaved: (data: WorkLog) => void
        onDeleted: () => void
    }

    let { log, caseId, onSaved, onDeleted }: Props = $props()
    let isEditMode = $state(false)
    let copiedData = $state(log)
    const selfId = getSelfId()

    function _onSaved(newValue: WorkLog) {
        copiedData = newValue
        isEditMode = false

        onSaved(newValue)
    }

    async function onDeleteClicked(e: Event) {
        e.preventDefault()
        e.stopPropagation()

        const confirmed = confirm(
            'Are you sure you want to delete this work log? This action cannot be undone.',
        )

        if (confirmed) {
            const resp = await WorkLogServices.delete(copiedData.id)
            if (resp.error) {
                alert(resp.message)
                return
            }

            alert('Work log has been deleted successfully')
            onDeleted()
        }
    }

    const hrs = $derived(roundTo(copiedData.duration / 60, 2))

    function formater(startedAt: Date, endedAt: Date) {
        const st = startedAt.toLocaleString('en-US', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            hour12: false,
        })

        let year: '2-digit' | undefined =
            startedAt.getFullYear() === endedAt.getFullYear()
                ? undefined
                : '2-digit'
        let month: '2-digit' | undefined =
            startedAt.getMonth() === endedAt.getMonth() ? undefined : '2-digit'
        let day: '2-digit' | undefined =
            startedAt.getDate() === endedAt.getDate() ? undefined : '2-digit'

        let ed = ''
        if (year === undefined && month === undefined && day === undefined) {
            ed = endedAt.toLocaleString('en-US', {
                year,
                month,
                day,
                hour: '2-digit',
                minute: '2-digit',
                hour12: false,
            })
        } else {
            ed = endedAt.toLocaleString('en-US', {
                year: '2-digit',
                month: '2-digit',
                day: '2-digit',
                hour: '2-digit',
                minute: '2-digit',
                hour12: false,
            })
        }

        return `${st} ~ ${ed}`
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
            onSaved={_onSaved}
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
            <p>{formater(copiedData.startedAt, copiedData.endedAt)}</p>
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

        <div class="flex h-fit flex-1/12 flex-row justify-end gap-2">
            {#if !copiedData.isCollaborative}
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
            {/if}
            {#if copiedData.user.id === selfId}
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
            {/if}

            <div class="flex h-fit flex-1/12 flex-row justify-end gap-2">
                &nbsp;
            </div>
        </div>
    </div>
{/if}
