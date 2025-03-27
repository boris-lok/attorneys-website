<script lang="ts">
    import Markdown from '@magidoc/plugin-svelte-marked'
    import { HomeServices } from '$lib/services/home.service'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'

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
                }),
            )
            .subscribe({
                error: console.error,
            })
    }

    $effect(() => {
        fetchData()
    })
</script>

<div class="relative flex flex-col md:flex-row md:items-center w-full">
    <div class="hidden overflow-clip md:block w-[50%]">
        <img alt="home-bg" src="https://images.pexels.com/photos/1324803/pexels-photo-1324803.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"/>
    </div>

    <div
        class="prose w-full md:min-w-[50%] md:max-w-[50%] md:px-16 px-6 pt-16 md:pt-0 md:animate-[--right-to-left_1s_ease-in-out]"
    >
        <Markdown source={content}></Markdown>
    </div>
</div>
