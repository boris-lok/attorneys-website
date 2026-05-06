<script lang="ts">
    import type { PageProps } from './$types'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import {
        type PendingWorkLog as PendingWorkLogType,
        type WorkLog as WorkLogType,
        WorkLogServices,
    } from '$lib/services/work_log.service'
    import WorkLog from '$lib/components/WorkLog.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'
    import { getSelfId, getSelfName } from '$lib/utils'
    import PendingWorkLog from '$lib/components/PendingWorkLog.svelte'

    const selfId = getSelfId()
    const selfName = getSelfName()

    let { data }: PageProps = $props()
    let id = data.id
    let logs: WorkLogType[] = $state([])
    let pendingLogs = $derived.by(() => {
        return logs
            .filter((log) => {
                const collaborators = log.collaborators.filter(
                    (collaborator) =>
                        collaborator.status === 'pending' &&
                        collaborator.userId === selfId,
                )
                return collaborators.length > 0
            })
            .map((log) => {
                const p: PendingWorkLogType = {
                    id: log.id,
                    startedAt: log.startedAt,
                    endedAt: log.endedAt,
                    duration: log.duration,
                    description: log.description,
                    user: {
                        id: selfId,
                        name: selfName,
                    },
                }

                return p
            })
    })
    let isLoading = $state(false)
    let isCreated = $state(false)

    function appendCase(log: WorkLogType) {
        logs = [log, ...logs]
        isCreated = false
    }

    async function fetchWorkLogs() {
        isLoading = true
        const resp = await WorkLogServices.list(id)
        isLoading = false
        if (resp.error) {
            console.error(resp.message)
            return
        }

        logs = resp.logs
    }

    function editStatus(
        id: string,
        status: 'accepted' | 'pending' | 'rejected',
    ) {
        let c = logs.find((l) => l.id === id)
        if (!c) return
        let collaborators = [...c.collaborators]
        let collaborator = collaborators.find(
            (collaborator) => collaborator.userId === selfId,
        )
        if (!collaborator) return
        collaborator.status = status
        c.collaborators = collaborators
    }

    $effect(() => {
        fetchWorkLogs()
    })
</script>

{#if isLoading}
    <Loading />
{/if}

<main>
    {#if isCreated}
        <div>
            <WorkLogEditor
                onClosed={() => (isCreated = false)}
                caseId={id}
                onSaved={appendCase}
            />
        </div>
    {:else}
        <div
            class="my-2 flex h-16 flex-row items-center justify-end gap-2 px-4"
        >
            <button class="cursor-pointer" onclick={() => (isCreated = true)}>
                <IconifyIcon
                    class="h-4 w-4 md:h-6 md:w-6"
                    icon="solar:add-folder-broken"
                />
            </button>
        </div>
    {/if}

    {#if pendingLogs.length > 0}
        <p class="px-5 text-xl font-semibold">Pending Logs</p>
        <div class="md:m-4 md:rounded md:shadow">
            <div
                class="hidden rounded-t px-4 md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-orange-300"
            >
                <p class="text-md flex-5/12 py-3 text-left font-bold">
                    Description
                </p>
                <p class="text-md flex-4/12 py-3 text-left font-bold">Period</p>
                <p class="text-md flex-1/12 py-3 text-left font-bold">
                    Used Hrs
                </p>
                <p class="text-md flex-2/12 py-3 text-left font-bold">&nbsp;</p>
            </div>
            <div class="h-fit max-h-96 w-full overflow-y-auto">
                {#each pendingLogs as log, i (log.id)}
                    <PendingWorkLog
                        {...log}
                        onDone={(e) => editStatus(log.id, e)}
                    />
                    {#if i < logs.length - 1}
                        <div class="mx-2 hidden h-[1px] bg-gray-200 md:block">
                            &nbsp;
                        </div>
                    {/if}
                {/each}
            </div>
        </div>
    {/if}

    {#if logs.length > 0}
        <p class="mt-16 px-5 text-xl font-semibold">Work Logs</p>
        <div class="md:m-4 md:rounded md:shadow">
            <div
                class="hidden rounded-t px-4 md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300"
            >
                <p class="text-md flex-5/12 py-3 text-left font-bold">
                    Description
                </p>
                <p class="text-md flex-2/12 py-3 text-left font-bold">Period</p>
                <p class="text-md flex-1/12 py-3 text-left font-bold">
                    Used Hrs
                </p>
                <p class="text-md flex-2/12 py-3 text-left font-bold">
                    Participants
                </p>
                <p class="text-md flex-1/12 py-3 text-left font-bold">&nbsp;</p>
            </div>
            <div class="h-fit max-h-96 w-full overflow-y-auto">
                {#each logs as log, i (log.id)}
                    <WorkLog
                        caseId={id}
                        {log}
                        onSaved={(e) => (logs[i] = e)}
                        onDeleted={() =>
                            (logs = logs.filter((l) => l.id !== log.id))}
                    />
                    {#if i < logs.length - 1}
                        <div class="mx-2 hidden h-[1px] bg-gray-200 md:block">
                            &nbsp;
                        </div>
                    {/if}
                {/each}
            </div>
        </div>
    {/if}
</main>
