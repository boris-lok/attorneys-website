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
        options: () => Promise<Options>
        onInput?: (s: string) => void
        onSelect?: (opt: Option) => void
        onBlur?: (s: string) => void
    }

    let {
        label,
        name,
        value = '',
        options,
        onInput,
        onBlur,
        onSelect
    }: Props = $props()

    let opts: Options = $state([])
    let draft = $state(value)
    let filteredOpts: Options = $derived.by(() => {
        const k = draft.toLowerCase()
        return opts.filter((opt) =>
            opt.value.toLowerCase().includes(k)
        )
    })
    let open = $state(false)
    let selected = false
    let disposed = false
    let loadId = 0

    function _onInput(e: Event & { currentTarget: HTMLInputElement }) {
        selected = false
        draft = e.currentTarget.value
        open = true
        onInput?.(draft)
    }

    function _onBlur(e: Event & { currentTarget: HTMLInputElement }) {
        open = false

        if (selected) {
            selected = false
            return
        }

        onBlur?.(e.currentTarget.value)
    }

    function _onSelect(opt: Option) {
        draft = opt.value
        open = false
        selected = true
        onSelect?.(opt)
    }

    function _onFocus() {
        open = true
        _ensureOptionsLoaded()
    }

    async function _ensureOptionsLoaded() {
        const id = ++loadId

        const res = await options()

        if (disposed || id !== loadId) return

        opts = res
    }

    $effect(() => {
        draft = value
    })

    $effect(() => {
        return () => {
            disposed = true
        }
    })
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
        onkeydown={e => {
            if (e.key === 'Escape') {
                open = false
            }
        }}
        class="h-8 w-full border-b border-gray-300 px-2 py-1 leading-tight text-gray-700 focus:border-blue-500 focus:outline-none"
        value={draft}
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
                tabindex="-1"
                onpointerdown={(e) => {
                    e.preventDefault()
                    _onSelect(opt)
                }}
            >
                {opt.value}
            </div>
        {/each}
    </div>
</div>
