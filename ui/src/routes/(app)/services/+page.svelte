<script lang="ts">
    import ServiceList from '$lib/components/ServiceList.svelte'
</script>

<div class="relative flex flex-col md:flex-row md:items-center">
    <div class="relative flex w-full flex-col md:max-w-6xl mx-auto">
        <p
            class="mb-8 px-4 pt-16 text-center text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16"
        >
            法律服務項目
        </p>
        <ServiceList isAdmin={false} />
    </div>
</div>