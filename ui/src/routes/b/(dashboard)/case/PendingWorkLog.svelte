<script lang="ts">
    import { WorkLogServices } from '$lib/services/workLog.service'
    import { dateRangeFormatter, roundTo } from '$lib/utils'
    import IconifyIcon from '@iconify/svelte'
    import type { PendingWorkLog } from '$lib/types'

    type Props = PendingWorkLog & {
        onDone?: (status: 'accepted' | 'rejected') => void
    }

    let { onDone, ...rest }: Props = $props()
    const hrs = $derived(roundTo(rest.duration / 60, 2))
    let isLoading = $state(false)

    async function onClicked(status: 'accepted' | 'rejected') {
        if (isLoading) return

        isLoading = true
        if (status === 'accepted') {
            await WorkLogServices.accept(rest.id)
        } else {
            await WorkLogServices.reject(rest.id)
        }
        isLoading = false

        onDone?.(status)
    }
</script>

<div
    class="m-4 rounded p-4 shadow md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:rounded-none md:p-2 md:shadow-none md:hover:bg-gray-50"
>
    <div class="my-2 flex-5/12 font-semibold text-nowrap md:my-0 md:font-medium">
        {rest.description}
    </div>

    <div class="my-1 flex-4/12 text-sm text-gray-500 md:my-0 md:text-gray-700">
        <p>{dateRangeFormatter(rest.startedAt, rest.endedAt)}</p>
    </div>

    <div class="flex-1/12 text-sm">
        <span class="text-gray-600 md:hidden">Duration: </span>
        <span>{hrs} hrs</span>
    </div>

    <div class="flex h-fit flex-2/12 flex-row justify-end gap-2">
        {#if isLoading}
            <div>
                <IconifyIcon class="text-blue-500 h-6 w-6 mr-6" icon="svg-spinners:90-ring-with-bg" />
            </div>
        {:else}
            <button class="cursor-pointer" onclick={() => onClicked('accepted')} aria-label="Accept work log">
                <IconifyIcon
                    class="h-4 w-4 text-green-500 md:h-6 md:w-6"
                    icon="line-md:circle-to-confirm-circle-transition"
                />
            </button>

            <button class="cursor-pointer" onclick={() => onClicked('rejected')} aria-label="Reject work log">
                <IconifyIcon class="h-4 w-4 text-red-500 md:h-6 md:w-6" icon="line-md:close-circle" />
            </button>
        {/if}
    </div>
</div>
