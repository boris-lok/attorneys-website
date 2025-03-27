<script lang="ts">
    import { ServiceServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import IconifyIcon from '@iconify/svelte'

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)
    let selectedService = $state('')

    function onServiceClicked(id: string) {
        selectedService = id
    }

    function fetchData() {
        ServiceServices.list('zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    services = resp
                    console.log(services)
                }),
            )
            .subscribe({
                error: console.error,
            })
    }

    $effect(() => fetchData())
</script>

<div class="relative flex flex-col md:flex-row md:items-center">
    <div class="relative flex flex-1 flex-col">
        <p
            class="px-4 pt-16 text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16"
        >
            法律服務項目
        </p>
        <div class="relative mt-8 flex flex-col md:flex-row md:items-center">
            <div class="flex-1">
                <div class="px-4 md:px-8 lg:px-16">
                    {#each services as service}
                        <button
                            class="my-4 flex w-full cursor-pointer flex-col overflow-clip rounded border border-green-700 px-4 py-4 text-left"
                            onclick={() => onServiceClicked(service.id)}
                        >
                            <div
                                class="group flex flex-row items-center justify-between"
                                class:show={selectedService == service.id}
                            >
                                <p
                                    class="text-lg font-bold text-[var(--primary-color)]"
                                >
                                    {service.data.title}
                                </p>
                                <IconifyIcon
                                    icon="ooui:collapse"
                                    class="h-4 w-4 rotate-180 text-[var(--primary-color)] duration-200 group-[.show]:rotate-0 md:transition-[rotate]"
                                />
                            </div>

                            <div
                                class="mt-1 h-0 w-0 bg-green-800 [&.show]:my-4 [&.show]:h-[1px] [&.show]:w-full"
                                class:show={selectedService == service.id}
                            ></div>

                            <div
                                class="prose max-h-0 overflow-hidden duration-300 ease-in-out md:transition-[max-height] [&.show]:max-h-64"
                                class:show={selectedService == service.id}
                            >
                                <Markdown source={service.data.data}></Markdown>
                            </div>
                        </button>
                    {/each}
                </div>
            </div>
        </div>
    </div>

    <div
        class="hidden h-[calc(100vh-64px-144px)] min-h-[48rem] flex-1 justify-end overflow-clip md:flex"
    >
        <img
            alt="services-bg"
            src="https://images.pexels.com/photos/5793979/pexels-photo-5793979.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"
        />
    </div>
</div>
