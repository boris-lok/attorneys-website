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
    <button class="cursor-pointer text-xl font-bold text-left w-36" onclick={() => isOpen = !isOpen}>
        類別
    </button>
    <div
        class="flex flex-col h-0 overflow-y-hidden [.active]:h-48 [.active]:overflow-y-scroll transition-[height] duration-200 lg:h-fit lg:overflow-y-visible"
        class:active={isOpen}>
        <button class="text-base w-36 cursor-pointer text-left hover:text-[var(--primary-color)] hover:text-xl"
                onclick={() => selectCategory(null)}>全部
        </button>
        {#each categories as category (category.id)}
            <button class="text-base w-36 cursor-pointer text-left hover:text-[var(--primary-color)] hover:text-xl"
                    onclick={() => selectCategory(category.id)}>{category.data.name}</button>
        {/each}
    </div>
</div>