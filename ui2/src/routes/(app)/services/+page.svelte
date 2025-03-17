<script lang="ts">
    import { ServicesServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'

    let services: ServiceData[] = $state([])
    let isLoading = false

    function fetchData() {
        ServicesServices.list('zh')
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

<div
    class="relative mx-4 my-8 grid grid-cols-1 gap-x-8 gap-y-16 md:mx-auto md:max-w-[var(--max-screen-width)] md:grid-cols-2"
>
    {#each services as service, i}
        <div class="w-full overflow-clip rounded shadow-(--box-shadow)">
            <p class="px-8 py-2 text-lg font-bold text-[var(--primary-color)]">
                {service.data.title}
            </p>
            <div class="prose px-8 py-2">
                <Markdown source={service.data.data}></Markdown>
            </div>
        </div>
    {/each}
</div>
