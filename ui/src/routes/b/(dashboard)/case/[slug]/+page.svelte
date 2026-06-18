<script lang="ts">
    import type { PageProps } from './$types'
    import WorkLogEditor from '../WorkLogEditor.svelte'
    import WorkLog from '../WorkLog.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/shared/Loading.svelte'
    import PendingWorkLog from '../PendingWorkLog.svelte'
    import DateTimePicker from '$lib/components/shared/DateTimePicker.svelte'
    import { CaseServices } from '$lib/services/case.service'
    import { useDateRange } from '$lib/composables/useDateRange.svelte'
    import { useWorkLog } from '$lib/composables/useWorkLog.svelte'
    import { WorkLogServices } from '$lib/services/workLog.service'
    import { toast } from '$lib/stores/toast.svelte'
    import { triggerDownload } from '$lib/utils'
    import { untrack } from 'svelte'


    let { data }: PageProps = $props()
    // svelte-ignore state_referenced_locally
    let caseId = data.caseId
    // svelte-ignore state_referenced_locally
    let user = data.user
    let range = useDateRange(90)
    let store = useWorkLog(caseId, user)
    let isCreated = $state(false)

    $effect(() => {
        untrack(() => store.fetch(range.startedAt, range.endedAt))
    })

    async function search(e: Event) {
        e.preventDefault()
        await store.fetch(range.startedAt, range.endedAt)
    }

    async function download() {
        const resp = await WorkLogServices.download(window.fetch, caseId, range.startedAt, range.endedAt)
        if (resp.error) {
            toast.show(resp.message, 'error')
            return
        }

        triggerDownload(resp.blob, `${new Date().toISOString().split('T')[0]}.xlsx`)
    }


    async function settle() {
        const resp = await CaseServices.settle(window.fetch, caseId)
        if (resp.error) {
            toast.show(resp.message, 'error')
            return
        }

        toast.show('Case settled successfully')
    }

</script>

{#if store.isLoading}
    <Loading />
{/if}


<main>
    {#if isCreated}
        <div class="px-2 md:px-8 md:mx-8 md:mt-8 md:rounded md:shadow md:py-4">
            <WorkLogEditor selfId={user.sub} selfName={user.nickname} onClosed={() => (isCreated = false)}
                           caseId={caseId}
                           onSaved={store.upsert} />
        </div>
    {:else}
        <div class="my-2 flex h-16 flex-row items-center justify-end gap-4 px-4">
            <button class="cursor-pointer" onclick={() => (isCreated = true)}>
                <IconifyIcon class="h-6 w-6" icon="tabler:library-plus" />
            </button>
        </div>
    {/if}

    {#if store.pendingLogs.length > 0}
        <p class="px-5 text-xl font-semibold">Pending Logs</p>
        <div class="md:m-4 md:rounded md:shadow">
            <div
                class="hidden rounded-t px-4 md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-orange-300"
            >
                <p class="text-md flex-5/12 py-3 text-left font-bold">Description</p>
                <p class="text-md flex-4/12 py-3 text-left font-bold">Period</p>
                <p class="text-md flex-1/12 py-3 text-left font-bold">Used Hrs</p>
                <p class="text-md flex-2/12 py-3 text-left font-bold">&nbsp;</p>
            </div>
            <div class="h-fit max-h-96 w-full overflow-y-auto">
                {#each store.pendingLogs as log, i (log.id)}
                    <PendingWorkLog {...log} onDone={(e) => store.editStatus(log.id, e)} />
                    {#if i < store.logs.length - 1}
                        <div class="mx-2 hidden h-px bg-gray-200 md:block">&nbsp;</div>
                    {/if}
                {/each}
            </div>
        </div>
    {/if}

    <p class="mt-16 px-5 text-xl font-semibold">Work Logs</p>

    <div
        class="flex flex-col items-center justify-center md:flex-row md:items-center md:justify-end md:gap-4 md:px-6"
    >
        <div class="m-2 flex flex-row items-center gap-4">
            <DateTimePicker
                date={range.startedAt}
                dateOnly={true}
                onChanged={(e) => range.set('startedAt', e)}
            />
            <span>~</span>
            <DateTimePicker
                date={range.endedAt}
                dateOnly={true}
                onChanged={(e) => range.set('endedAt', e)}
            />
        </div>

        <div class="flex flex-row items-center gap-2">
            <div class="relative group">
                <button class="cursor-pointer" onclick={download}>
                    <IconifyIcon class="h-6 w-6" icon="tabler:download" />

                </button>
                <p
                    class="absolute top-6 right-3 hidden rounded bg-black/50 px-2 py-1 text-white group-hover:block"
                >
                    Download
                </p>
            </div>

            <div class="relative group">
                <button class="group relative cursor-pointer" onclick={search}>
                    <IconifyIcon icon="tabler:file-search" class="h-6 w-6" />
                </button>
                <p
                    class="absolute top-6 right-3 hidden rounded bg-black/50 px-2 py-1 text-white group-hover:block"
                >
                    Search
                </p>
            </div>


            <div class="relative group">
                <button
                    class="group relative"
                    class:text-red-500={store.logs.length > 0}
                    class:cursor-pointer={store.logs.length > 0}
                    class:text-gray-300={store.logs.length === 0}
                    onclick={settle}
                    disabled={store.logs.length === 0}
                >
                    <IconifyIcon icon="tabler:align-box-right-top"
                                 class="h-6 w-6" />
                </button>
                <p
                    class="absolute top-6 right-3 hidden rounded bg-black/50 px-2 py-1 text-white group-hover:block"
                >
                    Settle
                </p>
            </div>

        </div>

    </div>

    <div class="md:m-4 md:rounded md:shadow">
        <div
            class="hidden rounded-t px-4 md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300"
        >
            <p class="text-md flex-5/12 py-3 text-left font-bold">Description</p>
            <p class="text-md flex-2/12 py-3 text-left font-bold">Period</p>
            <p class="text-md flex-1/12 py-3 text-left font-bold">Used Hrs</p>
            <p class="text-md flex-2/12 py-3 text-left font-bold">Participants</p>
            <p class="text-md flex-1/12 py-3 text-left font-bold">&nbsp;</p>
        </div>

        {#if store.logs.length > 0}
            <div class="h-fit max-h-96 w-full overflow-y-auto">
                {#each store.logs as log, i (log.id)}
                    <WorkLog
                        selfId={user.sub}
                        selfName={user.nickname}
                        caseId={caseId}
                        {log}
                        onSaved={store.upsert}
                        onDeleted={() => (store.remove(log.id))}
                    />
                    {#if i < store.logs.length - 1}
                        <div class="mx-2 hidden h-px bg-gray-200 md:block">&nbsp;</div>
                    {/if}
                {/each}
            </div>
        {:else}
            <div class="block h-full w-full">
                <p class="w-full py-3 text-center">No work logs found</p>
            </div>
        {/if}
    </div>
</main>
