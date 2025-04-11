<script lang="ts">

    import type { CategoryData } from '$lib/types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { CategoryService } from '$lib/services/category.service'
    import Loading from '$lib/components/common/Loading.svelte'

    let categories: CategoryData[] = $state([])
    let isLoading = $state(false)
    let language = 'zh'

    function fetchData() {
        CategoryService.list('zh')
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
<div>
    {#each categories as category (category.id)}
        <div>
            <p>{category.data.name}</p>
        </div>
    {/each}
</div>