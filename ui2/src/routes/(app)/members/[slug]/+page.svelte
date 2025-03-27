<script lang="ts">
    import type { PageProps } from './$types'
    import { startWithTap } from '$lib/utils'
    import { finalize, tap } from 'rxjs'
    import { MemberServices } from '$lib/services/member.service'
    import type { MemberData, ImageData } from '$lib/types'
    import Markdown from '@magidoc/plugin-svelte-marked'
    import Image from '$lib/components/common/Image.svelte'
    import IconifyIcon from '@iconify/svelte'

    let { data }: PageProps = $props()

    let isLoading = $state(false)
    let name = $state('')
    let description = $state('')
    let avatar: ImageData | undefined = $state(undefined)
    let seq = $state(0)

    function fetchData() {
        MemberServices.retrieve(data.id, 'zh')
            .pipe(
                startWithTap(() => (isLoading = true)),
                finalize(() => (isLoading = false)),
                tap((resp: MemberData | null) => {
                    if (resp) {
                        name = resp.data.name ?? ''
                        description = resp.data.description ?? ''
                        avatar = resp.avatar
                        seq = resp.seq ?? 0
                    }
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
    <div>
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
        <div class="prose">
            <Markdown source={description ?? ''}></Markdown>
        </div>
    </div>
{/if}
