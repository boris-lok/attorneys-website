<script lang="ts">
    import ServiceBox from '$lib/components/feature/ServiceBox.svelte'

    let { data } = $props()
</script>

<div class="relative flex flex-col md:flex-row md:items-center">
    <div class="relative mx-auto flex w-full flex-col md:max-w-6xl">
        <p
            class="mb-8 px-4 pt-16 text-center text-4xl font-bold text-(--primary-color) md:px-8 lg:px-16"
        >
            法律服務項目
        </p>
        <div
            class="relative mb-16 flex w-full flex-col items-center justify-center gap-x-16 gap-y-8 px-16 md:flex-row md:flex-wrap"
        >
            {#each data.data as service (service.id)}
                <div class="relative">
                    <ServiceBox
                        icon={service.data.icon}
                        title={service.data.title}
                        content={service.data.data}
                    />
                </div>
            {/each}
        </div>
    </div>
</div>
