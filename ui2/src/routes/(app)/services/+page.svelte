<script lang="ts">
    import { ServiceServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)
    let selectedServiceID = $state('')

    // handles serivce block clicked.
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
                <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
                <div class="group relative h-72 w-72 overflow-clip rounded-xl" onclick={() => onServiceClicked(service.id)} class:active={service.id === selectedServiceID}>
                    <div
                        class="relative h-full w-full rounded-xl border border-[var(--primary-color)] p-4 transition-[width,height]"
                    >
                        <p
                            class="flex h-full w-full items-center justify-center text-2xl font-bold text-[var(--primary-color)] max-sm:group-[.active]:opacity-20 group-hover:opacity-20"
                        >
                            {service.data.title}
                        </p>
                    </div>

                    <div class="absolute inset-0 opacity-0 bg-white group-hover:block max-sm:[&.active]:block max-sm:group-[.active]:opacity-80 group-hover:opacity-80"></div>
                    <div
                        class="absolute inset-0 hidden items-center justify-center p-4 group-hover:flex max-sm:group-[.active]:flex"
                    >
                        <Markdown source={service.data.data} />
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
