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
        class="flex flex-col bg-white rounded-lg shadow px-8 py-6 justify-around"
    >
        <h2 class="text-xl font-semibold text-gray-800">
            {title}
        </h2>

        <p class="text-gray-500 text-xs mt-4">
            發佈時間: {createdAt}
        </p>
    </div>
{:else}
    <a
        class="group flex flex-col bg-white rounded-lg shadow hover:shadow-xl px-8 py-6 transition duration-200 cursor-pointer justify-around"
        href="/articles/{id}"
    >
        <h2 class="text-xl font-semibold text-gray-800 group-hover:text-[var(--primary-color)] transition">
            {title}
        </h2>

        <p class="text-gray-500 text-xs mt-4">
            發佈時間: {createdAt}
        </p>
    </a>
{/if}
