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
        ;(async () => {
            isLoading = true
            await fetchData()
            isLoading = false
        })()
    })
</script>

{#if isLoading}
    <Loading />
{:else}
    <div class="my-8 px-4 md:mx-auto md:my-16 md:max-w-5xl md:px-8">
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
        <div class="prose mt-4 max-w-2xl md:max-w-3xl lg:max-w-4xl">
            <Markdown source={description ?? ''}></Markdown>
        </div>

        <div class="relative flex flex-row items-center justify-center">
            <button
                class="h-10 w-36 cursor-pointer rounded border transition-[background-color,font-size] duration-500 hover:border-[var(--primary-color)] hover:bg-[var(--primary-color)] hover:text-lg hover:text-white"
                onclick={onBackClicked}
                >返回
            </button>
        </div>
    </div>
{/if}
