<script lang="ts">
    import { ServiceServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)

    function onServiceClicked() {

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

<div class="relative flex flex-col">
    <p class="text-4xl px-16 pt-16 text-[var(--primary-color)] font-bold">法律服務項目</p>
    <div class="relative flex md:flex-row flex-col md:items-center mt-8">
        <div class="flex-1">
            <div class="px-16">
                {#each services as service}
                    <div class="overflow-clip rounded flex flex-col border border-green-700 my-4" onclick={() => onServiceClicked(service.id)}>
                        <p class="px-8 py-4 text-lg font-bold text-[var(--primary-color)]">
                            {service.data.title}
                        </p>

                        <div class="prose h-0 overflow-hidden [&.show]:h-fit px-8">
                            <Markdown source={service.data.data}></Markdown>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <div class="overflow-clip hidden md:block w-[50%] rounded-2xl">
            <img
                src="https://images.pexels.com/photos/4186912/pexels-photo-4186912.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"
                alt="home-bg" />
        </div>
    </div>
</div>
