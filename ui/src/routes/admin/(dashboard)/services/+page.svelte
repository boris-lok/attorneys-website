<script lang="ts">
    import { ServiceServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Icon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'
    import ServiceBox from '$lib/components/ServiceBox.svelte'

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)
    let selectedServiceID = $state('')

    // handles service block clicked.
    function onServiceClicked(id: string) {
        selectedServiceID = id
    }

    function fetchData() {
        ServiceServices.list('zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    services = resp
                    console.log(services)
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => fetchData())
</script>

{#if isLoading}
    <Loading />
{:else}
    <div>
        <div class="relative my-4 flex flex-row justify-end px-2">
            <a href="/admin/services/edit">
                <Icon height="24" icon="gridicons:create" width="24" />
            </a>
        </div>
        <div class="relative grid grid-cols-1 gap-x-8 gap-y-16 md:grid-cols-2 lg:grid-cols-3">
            {#each services as service, i}
                <div
                    class="w-fit relative"
                >
                    <button onclick={() => onServiceClicked(service.id)}>
                        <ServiceBox icon={service.data.icon} title={service.data.title}
                                    content={service.data.data} active={service.id === selectedServiceID} />
                    </button>
                    <div class="absolute top-2 right-2 z-10">
                        <a href="/admin/services/edit/{service.id}">
                            <Icon
                                icon="mingcute:edit-line"
                                width="24"
                                height="24"
                            />
                        </a>
                    </div>
                </div>
            {/each}
        </div>
    </div>

{/if}