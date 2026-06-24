<script lang="ts">
    import IconifyIcon from '@iconify/svelte'

    type InputProps = {
        currentPage: number
        totalPages: number
        onPageChanged: (page: number) => void
    }

    let { totalPages, currentPage, onPageChanged }: InputProps = $props()

    function goToPage(page: number) {
        if (page >= 0 && page < totalPages && page !== currentPage) {
            onPageChanged(page)
        }
    }
</script>

<div class="flex items-center justify-center space-x-8 py-4">
    <!-- Previous button -->
    <button
        class="flex cursor-pointer items-center rounded-md border border-gray-300 px-3 py-1 text-gray-700 hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50"
        disabled={currentPage === 0}
        onclick={() => goToPage(currentPage - 1)}
    >
        <IconifyIcon class="h-5 w-5" icon="mdi:chevron-left" />
        <span class="ml-1 hidden sm:inline">Previous</span>
    </button>

    <!-- Next button -->
    <button
        class="flex cursor-pointer items-center rounded-md border border-gray-300 px-3 py-1 text-gray-700 hover:bg-gray-100 disabled:cursor-not-allowed disabled:opacity-50"
        disabled={currentPage >= totalPages - 1}
        onclick={() => goToPage(currentPage + 1)}
    >
        <span class="mr-1 hidden sm:inline">Next</span>
        <IconifyIcon class="h-5 w-5" icon="mdi:chevron-right" />
    </button>
</div>
