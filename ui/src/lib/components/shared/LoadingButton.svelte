<script lang="ts">
    import IconifyIcon from '@iconify/svelte'

    type Variant = 'primary' | 'secondary' | 'danger'
    type Size = 'sm' | 'md' | 'lg'

    interface Props {
        isLoading?: boolean
        disabled?: boolean
        variant?: Variant
        size?: Size
        loadingText?: string
        type?: 'button' | 'submit' | 'reset'
        onclick?: () => void
        children: import().Snippet
    }

    let {
        isLoading = false,
        disabled = false,
        variant = 'primary',
        size = 'md',
        loadingText,
        type = 'button',
        onclick,
        children,
    }: Props = $props()

    const base =
        'inline-flex items-center justify-center gap-2 font-semibold rounded transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:cursor-not-allowed'

    const variants: Record<Variant, string> = {
        primary:
            'bg-blue-500 text-white hover:bg-blue-600 focus-visible:ring-blue-500 disabled:bg-blue-300',
        secondary:
            'bg-gray-100 text-gray-800 hover:bg-gray-200 focus-visible:ring-gray-400 disabled:bg-gray-50 disabled:text-gray-400',
        danger: 'bg-red-500 text-white hover:bg-red-600 focus-visible:ring-red-500 disabled:bg-red-300',
    }

    const sizes: Record<Size, string> = {
        sm: 'px-3 py-1.5 text-sm',
        md: 'px-4 py-2 text-sm',
        lg: 'px-6 py-3 text-base',
    }

    const spinnerSizes: Record<Size, string> = {
        sm: 'size-3.5',
        md: 'size-4',
        lg: 'size-5',
    }
</script>

<button
    {type}
    class="{base} {variants[variant]} {sizes[size]}"
    disabled={disabled || isLoading}
    aria-busy={isLoading}
    {onclick}
>
    {#if isLoading}
        <IconifyIcon icon="svg-spinners:90-ring-with-bg" class={spinnerSizes[size]} />
        {#if loadingText}
            <span>{loadingText}</span>
        {:else}
            {@render children()}
        {/if}
    {:else}
        {@render children()}
    {/if}
</button>
