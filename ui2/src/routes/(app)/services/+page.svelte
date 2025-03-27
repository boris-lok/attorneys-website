<script lang="ts">
    import { ServiceServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)

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
    <div class="relative flex w-full flex-col">
        <p
            class="mb-8 px-4 pt-16 text-center text-4xl font-bold text-[var(--primary-color)] md:px-8 lg:px-16"
        >
            法律服務項目
        </p>
        <div
            class="relative mb-16 flex w-full flex-col items-center justify-center gap-x-16 gap-y-8 px-16 md:flex-row md:flex-wrap"
        >
            {#each services as service}
                <div class="group relative h-72 w-72 overflow-clip rounded-xl">
                    <div
                        class="relative h-full w-full rounded-xl border border-[var(--primary-color)] p-4 transition-[width,height]"
                    >
                        <p
                            class="flex h-full w-full items-center justify-center text-2xl font-bold text-[var(--primary-color)]"
                        >
                            {service.data.title}
                        </p>
                    </div>
                    <div
                        class="absolute inset-0 hidden items-center justify-center p-4 group-hover:flex group-hover:backdrop-blur-sm"
                    >
                        <Markdown source={service.data.data} />
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
