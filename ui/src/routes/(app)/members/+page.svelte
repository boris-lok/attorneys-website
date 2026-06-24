<script lang="ts">
    import Image from '$lib/components/shared/Image.svelte'
    import IconifyIcon from '@iconify/svelte'

    let { data } = $props()
</script>

<div class="relative mt-8 flex flex-col md:mt-16 md:flex-row md:items-center">
    <div class="relative flex w-full flex-col gap-4">
        <p
            class="w-full px-4 text-center text-4xl font-bold text-(--primary-color) md:px-8 lg:px-16"
        >
            本所成員
        </p>
        <div
            class="relative mt-4 flex flex-col gap-8 px-4 md:flex-row md:justify-center md:px-24 lg:px-48"
        >
            {#each data.data as member (member.id)}
                <div
                    class="flex h-36 w-full flex-row justify-between rounded px-4 py-4 shadow-md md:w-84 lg:w-96"
                >
                    <a href="/members/{member.id}" class="flex flex-row items-center gap-4">
                        {#if member.avatar}
                            <div class="h-24 w-24">
                                <Image alt={member.name} image={member.avatar} />
                            </div>
                        {:else}
                            <IconifyIcon icon="tabler:user-circle" class="h-24 w-24" />
                        {/if}
                        <p class="text-2xl">{member.name}</p>
                    </a>
                </div>
            {/each}
        </div>
    </div>
</div>
