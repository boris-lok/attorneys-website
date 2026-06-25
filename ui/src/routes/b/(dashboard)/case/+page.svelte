<script lang="ts">
    import type { CaseData } from '$lib/types'
    import Case from './Case.svelte'
    import IconifyIcon from '@iconify/svelte'
    import CaseEditor from './CaseEditor.svelte'
    import type { PageProps } from './$types'

    let { data }: PageProps = $props()

    // data is only refreshed when SvelteKit re-runs the *load* function
    // When that happens, the page will re-render and the cases are re-initialized
    // svelte-ignore state_referenced_locally
    let allCases = $state(data.cases)
    let cases: CaseData[] = $derived(allCases.filter((e) => !e.closed))
    let closedCases = $derived(allCases.filter((e) => e.closed))
    // The state uses to control the creation
    // If it is true, the editor is open, else show the creation icon
    let isCreating = $state(false)

    // insert a new case or update an existing one
    function upsertCase(c: CaseData) {
        const exist = cases.some((e) => e.id === c.id)
        allCases = exist ? allCases.map((e) => (e.id === c.id ? c : e)) : [c, ...allCases]
    }

    // close the case
    function closeCase(id: string) {
        allCases = allCases.map((e) => (e.id === id ? { ...e, closed: true } : e))
    }

    // remove the case from the list
    function removeCase(id: string) {
        allCases = allCases.filter((e) => e.id !== id)
    }
</script>

{#snippet tableHeader(showActions: boolean)}
    <div
        class="hidden rounded-t md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300 md:p-2 md:gap-4"
    >
        <p class="text-md flex-3/12 py-3 text-left font-bold">Case Name</p>
        <p class="text-md flex-2/12 py-3 text-left font-bold">Period</p>
        <p class="text-md flex-2/12 py-3 text-left font-bold text-nowrap">Used Hrs</p>
        <p class="text-md flex-1/12 py-3 text-left font-bold text-nowrap">Next Billing</p>
        <p
            class="text-md py-3 text-left font-bold text-nowrap {showActions
                ? 'flex-1/12'
                : 'flex-auto'}"
        >
            Last Billing
        </p>
        {#if showActions}
            <p class="text-md flex-auto py-3 text-left font-bold w-24">&nbsp;</p>
        {/if}
    </div>
{/snippet}

<main>
    {#if isCreating}
        <div class="mx-4 my-4 flex items-center justify-center rounded px-8 shadow">
            <CaseEditor
                onClosed={() => (isCreating = false)}
                onSaved={(e) => {
                    isCreating = false
                    upsertCase(e)
                }}
            />
        </div>
    {:else}
        <div class="my-2 flex h-16 flex-row items-center justify-end gap-2 px-4">
            <button class="cursor-pointer" onclick={() => (isCreating = true)}>
                <IconifyIcon class="h-6 w-6" icon="tabler:library-plus" />
            </button>
        </div>
    {/if}

    <div class="md:m-4 md:rounded md:shadow">
        {@render tableHeader(true)}
        <div class="overflow-y-auto max-h-56">
            {#each cases as c, i (c.id)}
                <Case
                    {...c}
                    onSaved={upsertCase}
                    onDeleted={() => removeCase(c.id)}
                    editable
                    onClosed={() => closeCase(c.id)}
                />
                {#if i < cases.length - 1}
                    <div class="mx-2 hidden h-px bg-gray-200 md:block">&nbsp;</div>
                {/if}
            {/each}
        </div>
    </div>

    {#if closedCases.length > 0}
        <div class="mt-12">
            <p class="font-bold text-md lg:text-xl px-4">Closed Cases</p>
        </div>

        <div class="md:m-4 md:rounded md:shadow">
            {@render tableHeader(false)}
            <div class="overflow-y-auto max-h-56">
                {#each closedCases as c, i (c.id)}
                    <Case {...c} />
                    {#if i < closedCases.length - 1}
                        <div class="mx-2 hidden h-px bg-gray-200 md:block">&nbsp;</div>
                    {/if}
                {/each}
            </div>
        </div>
    {/if}
</main>
