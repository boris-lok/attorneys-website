<script lang="ts">
    import type { CategoryData } from '$lib/types'

    type InputProps = {
        categories: CategoryData[]
        selectedCategoryId: string | null
        onChanged: (categoryId: string | null) => void
    }

    let { categories, selectedCategoryId, onChanged }: InputProps = $props()

    let isOpen = $state(false)

    function selectCategory(categoryId: string | null) {
        selectedCategoryId = categoryId
    }

    $effect(() => onChanged(selectedCategoryId))
</script>

<div class="flex flex-col gap-2 px-8">
    <button
        class="w-36 cursor-pointer text-left text-xl font-bold"
        onclick={() => (isOpen = !isOpen)}
    >
        類別
    </button>
    <div
        class="flex h-0 flex-col overflow-y-hidden transition-[height] duration-200 lg:h-fit lg:overflow-y-visible [.active]:h-48 [.active]:overflow-y-scroll"
        class:active={isOpen}
    >
        <button
            class="w-36 cursor-pointer text-left text-base hover:text-xl hover:text-[var(--primary-color)]"
            onclick={() => selectCategory(null)}
            >全部
        </button>
        {#each categories as category (category.id)}
            <button
                class="w-36 cursor-pointer text-left text-base hover:text-xl hover:text-[var(--primary-color)]"
                onclick={() => selectCategory(category.id)}
                >{category.data.name}</button
            >
        {/each}
    </div>
</div>
