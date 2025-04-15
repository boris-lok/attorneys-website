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
            currentPage = page
        }
    }

    $effect(() => onPageChanged(currentPage))
</script>

<div class="flex items-center justify-center space-x-8 py-4">
    <!-- Previous button -->
    <button
        class="flex items-center px-3 py-1 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
        disabled={currentPage === 0}
        on:click={() => goToPage(currentPage - 1)}
    >
        <IconifyIcon class="h-5 w-5" icon="mdi:chevron-left" />
        <span class="hidden sm:inline ml-1">Previous</span>
    </button>

    <!-- Next button -->
    <button
        class="flex items-center px-3 py-1 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
        disabled={currentPage >= totalPages - 1}
        on:click={() => goToPage(currentPage + 1)}
    >
        <span class="hidden sm:inline mr-1">Next</span>
        <IconifyIcon class="h-5 w-5" icon="mdi:chevron-right" />
    </button>
</div>