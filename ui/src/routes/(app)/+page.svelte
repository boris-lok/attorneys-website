<script lang="ts">
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { HomeServices } from '$lib/services/home.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import Loading from '$lib/components/common/Loading.svelte'

    // The content of home page
    let content = $state('')
    // The loading statue of retrieving the content from API.
    let isLoading = $state(false)

    // fetch the content from API.
    function fetchData() {
        HomeServices.list('zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp) => {
                    if (resp.length === 0) {
                        content = ''
                    } else {
                        content = resp[0].data.data
                    }
                    console.log(resp)
                })
            )
            .subscribe({
                error: console.error
            })
    }

    $effect(() => {
        fetchData()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <div
        class="prose w-full px-6 pt-16 md:min-w-2xl lg:min-w-3xl md:animate-[--right-to-left_1s_ease-in-out] md:px-16 md:pt-0 mx-auto md:mt-16"
    >
        <Markdown source={content}></Markdown>
    </div>
{/if}
