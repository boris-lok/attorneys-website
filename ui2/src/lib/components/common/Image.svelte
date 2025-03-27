<script lang="ts">
    import type { ImageData } from '$lib/types'

    type InputProps = {
        alt: string
        image: ImageData | string
    }

    let { alt, image, size = 96 }: InputProps = $props()

    // Check if we use debug mode
    const debug = import.meta.env.VITE_DEBUG === 'true'

    let lgImage = $state(
        typeof image !== 'string' ? organizeURL(image.large_image) : '',
    )
    let smImage = $state(
        organizeURL(typeof image !== 'string' ? image.small_image : image),
    )

    function organizeURL(path: string) {
        return debug ? `http://localhost/images/${path}` : path
    }
</script>

<div class="overflow-clip">
    <picture>
        {#if typeof image !== 'string'}
            <source
                media="(min-width: 768px)"
                srcset={lgImage}
                width="256"
                height="256"
            />
        {/if}
        <img {alt} class="h-full w-full rounded-[50%]" src={smImage} />
    </picture>
</div>
