<script lang="ts">
    export type Option = {
        key: string
        value: string
    }
    export type Options = Option[]

    type Props = {
        label?: string
        name: string
        value?: string
        keywordFilter?: (keyword: string, opt: Option) => boolean
        validator?: (s: string) => boolean
        options: () => Promise<Options>
        onInput?: (s: string) => void
        onSelect?: (opt: Option) => void
        onBlur?: (s: string) => void
    }

    let {
        label,
        name,
        value = '',
        keywordFilter,
        options,
        onInput,
        onBlur,
        onSelect,
    }: Props = $props()
    let inputValue = $derived(value)

    let opts: Options = $state([])
    let keyword = $state(value)
    let filteredOpts: Options = $derived.by(() => {
        if (keywordFilter) {
            return opts.filter((opt) => keywordFilter(keyword, opt))
        }

        return opts
    })
    let open = $state(false)
    let optsLoaded = false

    function _onInput(e: Event & { currentTarget: HTMLInputElement }) {
        keyword = e.currentTarget.value
        open = true
        onInput?.(keyword)
    }

    function _onBlur(e: Event & { currentTarget: HTMLInputElement }) {
        open = false
        onBlur?.(e.currentTarget.value)
    }

    function _onSelect(opt: Option) {
        keyword = opt.value
        inputValue = opt.value
        open = false
        onSelect?.(opt)
    }

    function _onFocus() {
        open = true
        _ensureOptionsLoaded()
    }

    async function _ensureOptionsLoaded() {
        if (optsLoaded) return
        optsLoaded = true
        opts = await options()
    }
</script>

{#if label}
    <label for={name} class="text-md cursor-pointer font-medium">{label}</label>
{/if}

<div class="relative h-fit w-full">
    <input
        id={name}
        {name}
        oninput={_onInput}
        type="text"
        onblur={_onBlur}
        onfocus={_onFocus}
        class="h-8 w-full border-b border-gray-300 px-2 py-1 leading-tight text-gray-700 focus:border-blue-500 focus:outline-none"
        bind:value={inputValue}
    />

    <div
        class="absolute top-9 right-0 left-0 z-10 h-fit min-w-16 overflow-clip overflow-y-auto rounded bg-white px-2 shadow transition-[max-height] duration-200"
        role="listbox"
        class:max-h-0={!open}
        class:max-h-30={open}
    >
        {#each filteredOpts as opt (opt.key)}
            <div
                class="cursor-pointer p-1"
                role="option"
                aria-selected={opt.key === keyword}
                tabindex="-1"
                onpointerdown={(e) => {
                    e.preventDefault()
                    _onSelect?.(opt)
                }}
            >
                {opt.value}
            </div>
        {/each}
    </div>
</div>
