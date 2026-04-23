<script lang="ts">

    import type { PageProps } from './$types'
    import WorkLogEditor from '$lib/components/WorkLogEditor.svelte'
    import { type WorkLog as WorkLogType, WorkLogServices } from '$lib/services/work_log.service'
    import WorkLog from '$lib/components/WorkLog.svelte'
    import IconifyIcon from '@iconify/svelte'

    let { data }: PageProps = $props()
    let id = data.id
    let logs: WorkLogType[] = $state([])
    let isLoading = $state(false)
    let isCreated = $state(false)

    function appendCase(log: WorkLogType) {
        logs = [log, ...logs]
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

<div>
    {#each logs as log (log.id)}
        <WorkLog caseId={id} log={log} />
    {/each}
</div>