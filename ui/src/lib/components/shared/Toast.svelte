<script lang="ts">
    import { toast } from '$lib/stores/toast.svelte'
    import { fly } from 'svelte/transition'
    import IconifyIcon from '@iconify/svelte'
</script>

<div class="fixed bottom-4 md:bottom-8 w-full flex flex-col gap-1 items-center-safe">
    {#each toast.toasts as item (item.id)}
        <div
            class="toast toast--{item.type}"
            in:fly={{ x: 100, duration: 200 }}
            out:fly={{ x: 100, duration: 150, opacity: 0 }}
        >
            <p>{item.message}</p>
            <button class="text-black cursor-pointer" onclick={() => toast.dismiss(item.id)}>
                <IconifyIcon icon="solar:close-circle-line-duotone" />
            </button>
        </div>
    {/each}
</div>

<style lang="postcss">
    @reference '../../../app.css';

    .toast {
        @apply my-1 bg-gray-300/50 px-3 py-1 rounded sm:min-w-sm md:min-w-md max-w-full w-fit text-center flex flex-row justify-between;
    }

    .toast--success {
        @apply text-green-600;
    }

    .toast--error {
        @apply text-red-500;
    }
</style>