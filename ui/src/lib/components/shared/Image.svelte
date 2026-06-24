<script lang="ts">
    import type { ImageData } from '$lib/types'

    type InputProps = {
        alt: string
        image: ImageData | string
        size?: number
    }

    let { alt, image, size = 96 }: InputProps = $props()

    // Check if we use debug mode
    const debug = import.meta.env.VITE_DEBUG === 'true'

    let lgImage = $derived(typeof image !== 'string' ? organizeURL(image.lgImage) : '')
    let smImage = $derived(organizeURL(typeof image !== 'string' ? image.smImage : image))

    function organizeURL(path: string) {
        if (!path) return ''
        return debug ? `http://localhost/images/${path}` : path
    }
</script>

<div class="overflow-clip" style="width: {size}px; height: {size}px">
    <picture>
        {#if typeof image !== 'string'}
            <source media="(min-width: 768px)" srcset={lgImage} width="256" height="256" />
        {/if}
        <img {alt} class="h-full w-full rounded-full" src={smImage} width={size} height={size} />
    </picture>
</div>
