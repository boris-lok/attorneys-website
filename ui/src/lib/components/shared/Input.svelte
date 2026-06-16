<script lang="ts">
    import type { HTMLInputAttributes } from 'svelte/elements'

    type InputProps = HTMLInputAttributes & {
        label?: string
        variant?: 'default' | 'outlined'
        name: string
    }

    let {
        label,
        variant = 'default',
        value = $bindable(),
        id,
        name,
        ...props
    }: InputProps = $props()

    const uid = $props.id()
    const inputId = $derived(id ?? `${name}-${uid}`)
</script>

<div class="mb-4">
    {#if label}
        <label class="block text-sm font-bold text-gray-700" for={inputId}>{label}</label>
    {/if}
    <input
        {...props}
        class="base-classes"
        class:input-default={variant === 'default'}
        class:input-outlined={variant === 'outlined'}
        bind:value
        id={inputId}
        {name}
    />
</div>

<style lang="postcss">
    /* use reference to import global css for using *@apply* */
    @reference '../../../app.css';

    .base-classes {
        @apply w-full appearance-none px-1 py-2 leading-tight text-gray-700 focus:outline-none md:px-3;
    }

    .input-default {
        @apply rounded shadow focus:border-none focus:shadow-[0_0_0_3px_rgba(66,153,225,.5)];
    }

    .input-outlined {
        @apply border-b border-b-black focus:border-b-blue-500;
    }
</style>
