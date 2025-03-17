<script lang="ts">
    import { ServicesServices } from '$lib/services/service.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import type { ServiceData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import Icon from '@iconify/svelte'

    let services: ServiceData[] = $state([])
    let isLoading = $state(false)

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

{#if isLoading}
    <p>Loading...</p>
{:else}
    <div class="mx-4 my-8 md:mx-auto md:max-w-[var(--max-screen-width)]">
        <div class="relative my-4 flex flex-row justify-end px-2">
            <a href="/admin/services/edit">
                <Icon icon="gridicons:create" width="24" height="24" />
            </a>
        </div>
        <div class="relative grid grid-cols-1 gap-x-8 gap-y-16 md:grid-cols-2">
            {#each services as service, i}
                <div
                    class="flex w-full flex-row overflow-clip rounded shadow-(--box-shadow)"
                >
                    <div class="flex-auto">
                        <p
                            class="px-8 py-2 text-lg font-bold text-[var(--primary-color)]"
                        >
                            {service.data.title}
                        </p>
                        <div class="prose px-8 py-2">
                            <Markdown source={service.data.data}></Markdown>
                        </div>
                    </div>
                    <div class="px-2 py-2">
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
