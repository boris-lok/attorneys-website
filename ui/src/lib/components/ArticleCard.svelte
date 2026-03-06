<script lang="ts">
    type InputProps = {
        id: string
        title: string
        createdAt: string
        disabled?: boolean
    }

    let { id, title, createdAt, disabled = false }: InputProps = $props()
</script>

{#if disabled}
    <div
        class="flex flex-col justify-around rounded-lg bg-white px-8 py-6 shadow"
    >
        <h2 class="text-xl font-semibold text-gray-800">
            {title}
        </h2>

        <p class="mt-4 text-xs text-gray-500">
            發佈時間: {createdAt}
        </p>
    </div>
{:else}
    <a
        class="group flex cursor-pointer flex-col justify-around rounded-lg bg-white px-8 py-6 shadow transition duration-200 hover:shadow-xl"
        href="/articles/{id}"
    >
        <h2
            class="text-xl font-semibold text-gray-800 transition group-hover:text-[var(--primary-color)]"
        >
            {title}
        </h2>

        <p class="mt-4 text-xs text-gray-500">
            發佈時間: {createdAt}
        </p>
    </a>
{/if}
