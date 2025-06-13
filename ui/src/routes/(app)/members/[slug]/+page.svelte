<script lang="ts">
    import type { PageProps } from './$types'
    import { MemberServices } from '$lib/services/member.service'
    import type { ImageData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import Image from '$lib/components/common/Image.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { browser } from '$app/environment'
    import Loading from '$lib/components/common/Loading.svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let name = $state('')
    let description = $state('')
    let avatar: ImageData | undefined = $state(undefined)

    function onBackClicked() {
        if (browser) {
            window.history.back()
        }
    }

    async function fetchData() {
        const resp = await MemberServices.retrieve(data.id, 'zh')
        if (resp.error) {
            console.error(resp.message)
            return
        }

        name = resp.member?.data.name ?? ''
        description = resp.member?.data.description ?? ''
        avatar = resp.member?.avatar
    }

    $effect(() => {
        (async () => {
            isLoading = true
            await fetchData()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <div class="px-4 my-8 md:px-8 md:my-16 md:max-w-5xl md:mx-auto">
        <div
            class="relative flex flex-col items-center justify-between md:flex-row"
        >
            <p class="mb-8 text-3xl font-bold text-[var(--primary-color)]">
                {name}
            </p>
            {#if avatar}
                <div class="h-48 w-48">
                    <Image alt={name} image={avatar} />
                </div>
            {:else}
                <IconifyIcon icon="radix-icons:avatar" class="h-48 w-48" />
            {/if}
        </div>
        <div class="prose max-w-2xl md:max-w-3xl lg:max-w-4xl mt-4">
            <Markdown source={description ?? ''}></Markdown>
        </div>

        <div class="relative flex flex-row justify-center items-center">
            <button
                class="rounded border w-36 h-10 hover:border-[var(--primary-color)] cursor-pointer hover:bg-[var(--primary-color)] hover:text-white transition-[background-color,font-size] duration-500 hover:text-lg"
                onclick={onBackClicked}>返回
            </button>
        </div>
    </div>

{/if}