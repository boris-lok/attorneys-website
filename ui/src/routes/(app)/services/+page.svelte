<script lang="ts">
    import { ServiceServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
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


<div class="relative flex flex-col md:flex-row md:items-center">
    <div class="relative flex w-full flex-col md:max-w-6xl mx-auto">
        <p
            class="mb-8 px-4 pt-16 text-center text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16"
        >
            法律服務項目
        </p>
        <div
            class="relative mb-16 flex w-full flex-col items-center justify-center gap-x-16 gap-y-8 px-16 md:flex-row md:flex-wrap"
        >
            {#each services as service}
                <button onclick={() => onServiceClicked(service.id)}>
                    <ServiceBox icon={service.data.icon} title={service.data.title}
                                content={service.data.data} active={service.id === selectedServiceID} />
                </button>
            {/each}
        </div>
    </div>
</div>
