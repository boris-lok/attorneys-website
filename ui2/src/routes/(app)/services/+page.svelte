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
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => fetchData())
</script>

<div class="relative flex md:flex-row flex-col md:items-center">
    <div class="relative flex flex-col flex-1">
        <p class="text-4xl px-16 pt-16 text-[var(--primary-color)] font-bold">法律服務項目</p>
        <div class="relative flex md:flex-row flex-col md:items-center mt-8">
            <div class="flex-1">
                <div class="px-16">
                    {#each services as service}
                        <button class="overflow-clip rounded flex flex-col border border-green-700 my-4 py-4 px-4 w-full text-left cursor-pointer" onclick={() => onServiceClicked(service.id)}>
                            <div class="flex flex-row items-center justify-between group" class:show={selectedService == service.id}>
                                <p class="text-lg font-bold text-[var(--primary-color)]">
                                    {service.data.title}
                                </p>
                                <IconifyIcon icon="ooui:collapse" class="h-4 w-4 text-[var(--primary-color)] rotate-180 group-[.show]:rotate-0 md:transition-[rotate] duration-200"/>
                            </div>


                            <div class="h-0 bg-green-800 w-0 mt-1 [&.show]:h-[1px] [&.show]:w-full [&.show]:my-4" class:show={selectedService == service.id}></div>

                            <div class="prose max-h-0 overflow-hidden [&.show]:max-h-64 md:transition-[max-height] duration-300 ease-in-out" class:show={selectedService == service.id}>
                                <Markdown source={service.data.data}></Markdown>
                            </div>
                        </button>
                    {/each}
                </div>
            </div>
        </div>
    </div>

    <div class="overflow-clip hidden md:block flex-1 h-[48rem]">
        <img
            src="https://images.pexels.com/photos/4186912/pexels-photo-4186912.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"
            alt="home-bg" />
    </div>

</div>
