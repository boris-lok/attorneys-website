<script lang="ts">
    import type { ServiceData } from '$lib/types'
    import { ServiceServices } from '$lib/services/service.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import ServiceBox from '$lib/components/ServiceBox.svelte'
    import Icon from '@iconify/svelte'

    type InputProps = {
        isAdmin: boolean
    }

    let { isAdmin }: InputProps = $props()

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)
    let selectedServiceId: string | null = $state(null)

    function onServiceClicked(id: string) {
        selectedServiceId = id
    }

    async function fetchServices() {
        const resp = await ServiceServices.list('zh')
        if (resp.error) {
            console.error(resp.message)
            return []
        }

        return resp.services
    }

    $effect(() => {
        ;(async () => {
            isLoading = true
            services = await fetchServices()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <div
        class="relative mb-16 flex w-full flex-col items-center justify-center gap-x-16 gap-y-8 px-16 md:flex-row md:flex-wrap"
    >
        {#each services as service (service.id)}
            <div class="relative">
                <button onclick={() => onServiceClicked(service.id)}>
                    <ServiceBox
                        icon={service.data.icon}
                        title={service.data.title}
                        content={service.data.data}
                        active={service.id === selectedServiceId}
                    />
                </button>
                {#if isAdmin}
                    <div
                        class="absolute top-2 right-2 z-10 rounded p-1 opacity-10 hover:bg-gray-400/20 hover:opacity-100"
                    >
                        <a href="/admin/services/edit/{service.id}">
                            <Icon icon="mingcute:edit-line" width="24" height="24" />
                        </a>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
{/if}
