<script lang="ts">
    import MemberList from '$lib/components/MemberList.svelte'
</script>

<div class="relative flex flex-col md:flex-row md:items-center mt-8 md:mt-16">
    <div class="relative flex flex-col gap-4 w-full">
        <p
            class="px-4 text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16 w-full text-center"
        >
            本所成員
        </p>
        <div
            class="relative flex flex-col gap-8 px-4 md:flex-row md:px-24 lg:px-48 mt-4 md:justify-center"
        >
            <MemberList isAdmin={false} />
        </div>
    </div>
</div>