<script lang="ts">
    type InputProps = {
        label?: string
        name: string
        type: 'text' | 'password' | 'number'
        value: string | number
        variant?: 'default' | 'outlined'
        onInput: (
            event: Event & { currentTarget: EventTarget & HTMLInputElement }
        ) => void
    }

    let {
        label,
        name,
        type,
        value,
        onInput,
        variant = 'default'
    }: InputProps = $props()

    function typeAction(node: HTMLInputElement) {
        node.type = type
    }
</script>

<div class="mb-4">
    {#if label}
        <label class="block text-sm font-bold text-gray-700" for={name}
        >{label}</label
        >
    {/if}
    <input
        class="base-classes"
        class:default={variant === 'default'}
        class:outlined={variant === 'outlined'}
        id={name}
        {name}
        oninput={onInput}
        type={type}
        use:typeAction
        value={value}
    />
</div>

<style>
    /* use reference to import global css for using *@apply* */
    @reference '../../../app.css';

    .base-classes {
        @apply w-full appearance-none px-1 py-2 leading-tight text-gray-700 focus:outline-none md:px-3;
    }

    .default {
        @apply rounded shadow focus:border-none focus:shadow-[0_0_0_3px_rgba(66,153,225,.5)];
    }

    .outlined {
        @apply border-b border-b-black focus:border-b-blue-500;
    }
</style>
