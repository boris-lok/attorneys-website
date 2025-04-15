<script lang="ts">
    import type { CategoryData } from '$lib/types'

    type InputProps = {
        categories: CategoryData[]
        onChanged: (categoryId: string | null) => void
    }

    let { categories, onChanged }: InputProps = $props()

    let selectedCategoryId: string | null = $state(null)
    let isOpen = $state(false)

    function selectCategory(categoryId: string | null) {
        selectedCategoryId = categoryId
    }

    $effect(() => onChanged(selectedCategoryId))
</script>

<div class="flex flex-col gap-2 px-8">
    <button class="cursor-pointer" onclick={() => isOpen = !isOpen}>
        <p class="text-xl font-bold text-left w-36">類別</p>
    </button>
    <div
        class="flex flex-col h-0 overflow-y-hidden [.active]:h-48 [.active]:overflow-y-scroll transition-[height] duration-200 lg:h-fit lg:overflow-y-visible"
        class:active={isOpen}>
        {#each categories as category (category.id)}
            <button class="text-base w-36 cursor-pointer text-left hover:text-[var(--primary-color)] hover:text-xl"
                    onclick={() => selectCategory(category.id)}>{category.data.name}</button>
        {/each}
    </div>
</div>