<script lang="ts">
    import Icon from '@iconify/svelte'
    import ServiceList from '$lib/components/ServiceList.svelte'
</script>

<div>
    <div class="relative my-4 flex flex-row justify-end px-2">
        <a href="/admin/services/edit">
            <Icon height="24" icon="gridicons:create" width="24" />
        </a>
    </div>
    <ServiceList isAdmin={true} />
</div>