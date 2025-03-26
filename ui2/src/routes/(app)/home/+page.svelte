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

<div class="relative flex md:flex-row flex-col md:items-center">
    <div class="overflow-clip hidden md:block">
        <img
            class="h-[48rem]"
            src="https://images.pexels.com/photos/139195/pexels-photo-139195.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=2"
            alt="home-bg"
        />
    </div>

    <div class="prose mx-auto my-4 max-w-[var(--max-screen-width)] px-4 py-4 md:px-16 md:py-16">
        <Markdown source={content}></Markdown>
    </div>
</div>
