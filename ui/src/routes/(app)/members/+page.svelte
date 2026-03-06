<script lang="ts">
    import MemberList from '$lib/components/MemberList.svelte'
</script>

<div class="relative mt-8 flex flex-col md:mt-16 md:flex-row md:items-center">
    <div class="relative flex w-full flex-col gap-4">
        <p
            class="w-full px-4 text-center text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16"
        >
            本所成員
        </p>
        <div
            class="relative mt-4 flex flex-col gap-8 px-4 md:flex-row md:justify-center md:px-24 lg:px-48"
        >
            <MemberList isAdmin={false} />
        </div>
    </div>
</div>
