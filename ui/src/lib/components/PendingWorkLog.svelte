<script lang="ts">
    import type { PendingWorkLog } from '$lib/services/work_log.service'
    import { dateRangeFormatter, roundTo } from '$lib/utils'
    import IconifyIcon from '@iconify/svelte'

    type Props = PendingWorkLog & {
        onDone?: (status: 'accepted' | 'rejected') => void
    }

    let { onDone, ...rest }: Props = $props()
    let copiedData: PendingWorkLog = $state({ ...rest })
    const hrs = roundTo(copiedData.duration / 60, 2)

    function onAcceptClicked() {
        onDone?.('accepted')
    }

    function onRejectClicked() {
        onDone?.('rejected')
    }
</script>

<div
    class="m-4 rounded p-4 shadow md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:rounded-none md:p-2 md:shadow-none md:hover:bg-gray-50"
>
    <div
        class="my-2 flex-5/12 font-semibold text-nowrap md:my-0 md:font-medium"
    >
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
        <button class="cursor-pointer" onclick={onAcceptClicked}>
            <IconifyIcon
                class="h-4 w-4 text-green-500 md:h-6 md:w-6"
                icon="mdi:clipboard-tick-outline"
            />
        </button>

        <button class="cursor-pointer" onclick={onRejectClicked}>
            <IconifyIcon
                class="h-4 w-4 text-red-500 md:h-6 md:w-6"
                icon="iconamoon:file-close-light"
            />
        </button>
    </div>
</div>
