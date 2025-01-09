<script lang="ts">
    type InputProps = {
        label: string
        name: string
        type: 'text' | 'password' | 'number'
        value: string
        hasError: boolean
        onInput?: (
            event: Event & { currentTarget: EventTarget & HTMLInputElement },
        ) => void
    }

    let { label, name, type, value, hasError, onInput }: InputProps = $props()

    function typeAction(node: HTMLInputElement) {
        node.type = type
    }
</script>

<div class="mb-4">
    <label class="mb-2 block text-sm font-bold text-gray-700" for={name}
        >{label}</label
    >
    <input
        class="w-full appearance-none rounded border px-3 py-2 leading-tight text-gray-700 shadow focus:shadow-[0_0_0_3px_rgba(66,153,225,.5)] focus:outline-none"
        class:error={hasError}
        id={name}
        {name}
        oninput={onInput}
        type="text"
        use:typeAction
        {value}
    />
</div>
