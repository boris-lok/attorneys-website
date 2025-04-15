<script lang="ts">

    import type { CategoryData, Language } from '$lib/types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { CategoryService } from '$lib/services/category.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import Icon from '@iconify/svelte'
    import IconifyIcon from '@iconify/svelte'

    let categories: CategoryData[] = $state([])
    let isLoading = $state(false)
    let language: Language = 'zh'

    function fetchData() {
        CategoryService.list(language)
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    categories = resp
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
    <div class="relative my-4 flex flex-row justify-end px-2">
        <a href="/admin/categories/edit">
            <Icon height="24" icon="gridicons:create" width="24" />
        </a>
    </div>
    <div class="flex flex-wrap gap-4 justify-around">
        {#each categories as category (category.id)}
            <div class="rounded border w-36 h-36 flex flex-col items-center justify-center relative">
                {#if category.data.icon}
                    <IconifyIcon icon={category.data.icon} class="w-8 h-8" />
                {/if}
                <p>{category.data.name}</p>
                <div class="absolute p-2 top-0 right-0">
                    <a href="/admin/categories/edit/{category.id}">
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
{/if}