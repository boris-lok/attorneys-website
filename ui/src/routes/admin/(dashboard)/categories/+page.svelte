<script lang="ts">

    import type { CategoryData, Language } from '$lib/types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { CategoryService } from '$lib/services/category.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import Icon from '@iconify/svelte'

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

<Loading show={isLoading} />
<div class="relative my-4 flex flex-row justify-end px-2">
    <a href="/admin/categories/edit">
        <Icon icon="gridicons:create" width="24" height="24" />
    </a>
</div>
<div>
    {#each categories as category (category.id)}
        <div>
            <p>{category.data.name}</p>
        </div>
    {/each}
</div>