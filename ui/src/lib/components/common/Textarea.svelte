<script lang="ts">
    type TextareaProps = {
        label: string
        name: string
        value: string
        height?: string
        onInput?: (
            e: Event & { currentTarget: EventTarget & HTMLTextAreaElement },
        ) => void
    }

    let {
        label,
        name,
        value,
        onInput,
        height = 'h-96',
    }: TextareaProps = $props()
</script>

<div>
    <label class="mb-2 block text-sm font-medium text-gray-900" for={name}
        >{label}</label
    >
    <textarea
        bind:value
        class="block {height} w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500"
        id={name}
        {name}
        oninput={onInput}
    ></textarea>
</div>
