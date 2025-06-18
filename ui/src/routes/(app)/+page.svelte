<script lang="ts">
    import HomeContent from '$lib/components/HomeContent.svelte'
    import type { Language } from '$lib/types'

    let language: Language = 'zh'
</script>

<div class="md:animate-[--right-to-left_1s_ease-in-out]">
    <HomeContent language={language} />
</div>
