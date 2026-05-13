<script lang="ts">
    import type { PageProps } from './$types'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import {
        type PendingWorkLog as PendingWorkLogType,
        type WorkLog as WorkLogType,
        WorkLogServices
    } from '$lib/services/work_log.service'
    import WorkLog from '$lib/components/WorkLog.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'
    import { getSelfId, getSelfName } from '$lib/utils'
    import PendingWorkLog from '$lib/components/PendingWorkLog.svelte'
    import DateTimePicker from '$lib/components/DateTimePicker.svelte'
    import { untrack } from 'svelte'

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
                        collaborator.userId === selfId
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
                        name: selfName
                    }
                }

                return p
            })
    })
    let isLoading = $state(false)
    let isCreated = $state(false)
    let downloadLink: HTMLAnchorElement
    let startedAt = $state(
        setDateSuffix(
            new Date(Date.now() - 90 * 24 * 60 * 60 * 1000),
            0,
            0,
            0,
            0
        )
    )
    let endedAt = $state(setDateSuffix(new Date(), 23, 59, 59, 59))

    function setDateSuffix(
        date: Date,
        hrs: number,
        mins: number,
        s: number,
        ms: number
    ): Date {
        return new Date(
            date.getFullYear(),
            date.getMonth(),
            date.getDate(),
            hrs,
            mins,
            s,
            ms
        )
    }

    function onDateChanged(type: 'startedAt' | 'endedAt', date: Date) {
        if (type === 'startedAt') {
            startedAt = date
        } else if (type === 'endedAt') {
            endedAt = date
        }
    }

    function appendCase(log: WorkLogType) {
        logs = [log, ...logs]
        isCreated = false
    }

    async function fetchWorkLogs() {
        isLoading = true
        const resp = await WorkLogServices.list(id, startedAt, endedAt)
        isLoading = false
        if (resp.error) {
            console.error(resp.message)
            return
        }

        logs = resp.logs
    }

    async function search(e: Event) {
        e.preventDefault()
        e.stopPropagation()
        await fetchWorkLogs()
    }

    function editStatus(
        id: string,
        status: 'accepted' | 'pending' | 'rejected'
    ) {
        let c = logs.find((l) => l.id === id)
        if (!c) return
        let collaborators = [...c.collaborators]
        let collaborator = collaborators.find(
            (collaborator) => collaborator.userId === selfId
        )
        if (!collaborator) return
        collaborator.status = status
        c.collaborators = collaborators
    }

    async function download() {
        const resp = await WorkLogServices.download(id, startedAt, endedAt)
        if (resp.error) {
            console.error(resp.message)
            return
        }

        // Create download link
        const urlBlob = window.URL.createObjectURL(resp.blob)
        downloadLink.href = urlBlob
        downloadLink.download = `${new Date().toISOString().split('T')[0]}.xlsx`
        downloadLink.click()

        URL.revokeObjectURL(urlBlob)
    }

    $effect(() => {
        untrack(() => fetchWorkLogs())
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
            class="my-2 flex h-16 flex-row items-center justify-end gap-4 px-4"
        >
            <button class="cursor-pointer" onclick={() => (isCreated = true)}>
                <IconifyIcon class="h-6 w-6" icon="tabler:library-plus" />
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

    <p class="mt-16 px-5 text-xl font-semibold">Work Logs</p>

    <div
        class="flex flex-col items-center justify-center md:flex-row md:items-center md:justify-end md:px-6 md:gap-4"
    >
        <div class="m-2 flex flex-row items-center gap-4">
            <DateTimePicker
                date={startedAt}
                dateOnly={true}
                onChanged={(e) => onDateChanged('startedAt', e)}
            />
            <span>~</span>
            <DateTimePicker
                date={endedAt}
                dateOnly={true}
                onChanged={(e) => onDateChanged('endedAt', e)}
            />
        </div>


        <button class="cursor-pointer relative group" onclick={download}>
            <IconifyIcon class="h-6 w-6" icon="tabler:download" />
            <p
                class="absolute top-6 right-3 hidden rounded bg-black/50 px-2 py-1 text-white group-hover:block"
            >
                Download
            </p>
        </button>

        <button class="group relative cursor-pointer" onclick={search}>
            <IconifyIcon icon="tabler:file-search" class="h-6 w-6" />
            <p
                class="absolute top-6 right-3 hidden rounded bg-black/50 px-2 py-1 text-white group-hover:block"
            >
                Search
            </p>
        </button>
    </div>

    <div class="md:m-4 md:rounded md:shadow">
        <div
            class="hidden rounded-t px-4 md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300"
        >
            <p class="text-md flex-5/12 py-3 text-left font-bold">
                Description
            </p>
            <p class="text-md flex-2/12 py-3 text-left font-bold">Period</p>
            <p class="text-md flex-1/12 py-3 text-left font-bold">Used Hrs</p>
            <p class="text-md flex-2/12 py-3 text-left font-bold">
                Participants
            </p>
            <p class="text-md flex-1/12 py-3 text-left font-bold">&nbsp;</p>
        </div>

        {#if logs.length > 0}
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
        {:else}
            <div class="block h-full w-full">
                <p class="w-full py-3 text-center">No work logs found</p>
            </div>
        {/if}
    </div>

    <a bind:this={downloadLink} class="hidden"></a>
</main>
