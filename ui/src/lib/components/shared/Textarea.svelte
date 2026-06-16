<script lang="ts">
    import type { HTMLTextareaAttributes } from 'svelte/elements'

    type TextareaProps = HTMLTextareaAttributes & {
        label: string
        height?: string
    }

    let { label, value = $bindable(''), height = 'h-96', ...props }: TextareaProps = $props()

    const uid = $props.id()
</script>

<div>
    <label class="mb-2 block text-sm font-medium text-gray-900" for={uid}>{label}</label>
    <textarea
        {...props}
        bind:value
        class="base-classes {height}"
        id={uid}
    ></textarea>
</div>

<style lang="postcss">
    /* use reference to import global css for using *@apply* */
    @reference '../../../app.css';

    .base-classes {
        @apply w-full block rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500;
    }

</style>
