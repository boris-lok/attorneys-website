<script lang="ts">
    import type { PageProps } from './$types'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import {
        type WorkLog as WorkLogType,
        WorkLogServices,
    } from '$lib/services/work_log.service'
    import WorkLog from '$lib/components/WorkLog.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()
    let id = data.id
    let logs: WorkLogType[] = $state([])
    let isLoading = $state(false)
    let isCreated = $state(false)

    function appendCase(log: WorkLogType) {
        logs = [log, ...logs]
        isCreated = false
    }

    $effect(() => {
        const load = async () => {
            isLoading = true
            const resp = await WorkLogServices.list(id)
            isLoading = false
            if (resp.error) {
                console.error(resp.message)
                return
            }

            logs = resp.logs
        }

        load()
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
</main>

<div class="md:m-4 md:rounded md:shadow">
    <div
        class="hidden rounded-t md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300"
    >
        <p class="text-md flex-5/12 px-2 py-3 text-left font-bold">
            Description
        </p>
        <p class="text-md flex-2/12 px-2 py-3 text-left font-bold">
            Started At
        </p>
        <p class="text-md flex-1/12 px-2 py-3 text-left font-bold">Used Hrs</p>
        <p class="text-md flex-2/12 px-2 py-3 text-left font-bold">
            Participants
        </p>
        <p class="text-md flex-1/12 px-2 py-3 text-left font-bold">&nbsp;</p>
    </div>
    {#each logs as log, i (log.id)}
        <WorkLog caseId={id} {log} onSaved={(e) => (logs[i] = e)} />
        {#if i < logs.length - 1}
            <div class="mx-2 hidden h-[1px] bg-gray-200 md:block">&nbsp;</div>
        {/if}
    {/each}
</div>
