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

    let { label, name, value = '', keywordFilter, options, onInput, onBlur, onSelect }: Props = $props()
    let inputValue = $state(value)

    let opts: Options = $state([])
    let keyword = $state(value)
    let filteredOpts: Options = $derived.by(() => {
        if (keywordFilter) {
            return opts.filter(opt => keywordFilter(keyword, opt))
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
    <label for={name} class="cursor-pointer font-medium text-md">{label}</label>
{/if}

<div class="relative w-full h-fit">
    <input
        id={name}
        {name}
        oninput={_onInput}
        type="text"
        onblur={_onBlur}
        onfocus={_onFocus}
        class="border-b border-gray-300 focus:border-blue-500 focus:outline-none px-2 w-full py-1 leading-tight text-gray-700 h-8"
        bind:value={inputValue}
    />

    <div
        class="min-w-16 h-fit z-10 bg-white overflow-y-auto overflow-clip px-2 duration-200 transition-[max-height] absolute left-0 right-0 shadow rounded top-9"
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
            }}>{opt.value}</div>
        {/each}
    </div>
</div>
