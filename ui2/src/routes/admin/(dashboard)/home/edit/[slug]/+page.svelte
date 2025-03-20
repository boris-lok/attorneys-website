<script lang="ts">
    import type { PageProps } from './$types'
    import HomeEditor from '$lib/components/dashboard/HomeEditor.svelte'
    import { HomeServices } from '$lib/services/home.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let content = $state('')

    function fetchData() {
        console.log(data.id)
        HomeServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    content = resp?.data.data ?? ''
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
    <HomeEditor id={data.id} data={content} />
{/if}
