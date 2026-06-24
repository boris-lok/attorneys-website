<script lang="ts">
    import Markdown from '@magidoc/plugin-svelte-marked'

    let { data } = $props()
</script>

<div class="md:animate-[--right-to-left_1s_ease-in-out]">
    <div class="prose mx-auto my-6 w-full px-6 md:my-16 md:min-w-2xl md:px-16 lg:min-w-3xl">
        <Markdown source={data.data}></Markdown>
    </div>
</div>
