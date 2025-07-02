<script lang="ts">
    import Icon from '@iconify/svelte'
    import MemberList from '$lib/components/MemberList.svelte'
</script>

<div>
    <div class="relative my-4 flex flex-row justify-end px-2">
        <a href="/admin/members/edit">
            <Icon height="24" icon="gridicons:create" width="24" />
        </a>
    </div>
    <div class="relative flex flex-col gap-8 px-4 md:flex-row mt-4 md:justify-center">
        <MemberList isAdmin={true} />
    </div>
</div>
