<script lang="ts">
    import { WorkLogServices } from '$lib/services/workLog.service'
    import { dateRangeFormatter, roundTo } from '$lib/utils'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/shared/Loading.svelte'
    import type { PendingWorkLog } from '$lib/types'

    type Props = PendingWorkLog & {
        onDone?: (status: 'accepted' | 'rejected') => void
    }

    let { onDone, ...rest }: Props = $props()
    // svelte-ignore state_referenced_locally
    let copiedData: PendingWorkLog = $state({ ...rest })
    const hrs = roundTo(copiedData.duration / 60, 2)
    let isLoading = $state(false)

    async function onClicked(status: 'accepted' | 'rejected') {
        isLoading = true
        if (status === 'accepted') {
            await WorkLogServices.accept(copiedData.id)
        } else {
            await WorkLogServices.reject(copiedData.id)
        }
        isLoading = false

        onDone?.(status)
    }
</script>

{#if isLoading}
    <Loading />
{/if}

<div
    class="m-4 rounded p-4 shadow md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:rounded-none md:p-2 md:shadow-none md:hover:bg-gray-50"
>
    <div class="my-2 flex-5/12 font-semibold text-nowrap md:my-0 md:font-medium">
        {copiedData.description}
    </div>

    <div class="my-1 flex-4/12 text-sm text-gray-500 md:my-0 md:text-gray-700">
        <p>{dateRangeFormatter(copiedData.startedAt, copiedData.endedAt)}</p>
    </div>

    <div class="flex-1/12 text-sm">
        <span class="text-gray-600 md:hidden">Duration: </span>
        <span>{hrs} hrs</span>
    </div>

    <div class="flex h-fit flex-2/12 flex-row justify-end gap-2">
        <button class="cursor-pointer" onclick={() => onClicked('accepted')}>
            <IconifyIcon
                class="h-4 w-4 text-green-500 md:h-6 md:w-6"
                icon="line-md:circle-to-confirm-circle-transition"
            />
        </button>

        <button class="cursor-pointer" onclick={() => onClicked('rejected')}>
            <IconifyIcon class="h-4 w-4 text-red-500 md:h-6 md:w-6" icon="line-md:close-circle" />
        </button>
    </div>
</div>
