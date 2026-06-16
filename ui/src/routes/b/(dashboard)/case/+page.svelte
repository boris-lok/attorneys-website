<script lang="ts">
    import type { CaseData } from '$lib/types'
    import Case from '$lib/components/Case.svelte'
    import IconifyIcon from '@iconify/svelte'
    import CaseEditor from '$lib/components/CaseEditor.svelte'
    import type { PageProps } from './$types'

    let { data }: PageProps = $props()

    // data is only refreshed when SvelteKit re-runs the *load* function
    // When that happens, the page will re-render and the cases are re-initialized
    // svelte-ignore state_referenced_locally
    let cases: CaseData[] = $state(data.cases)
    // The state uses to control the creation
    // If it is true, the editor is open, else show the creation icon
    let isCreating = $state(false)

    // insert a new case or update an existing one
    function upsertCase(c: CaseData) {
        const exist = cases.some(e => e.id === c.id)
        cases = exist ?
            cases.map((e) => e.id === c.id ? c : e) :
            [c, ...cases]
    }

    // remove the case from the list
    function removeCase(id: string) {
        cases = cases.filter((e) => e.id !== id)
    }

</script>

<main>
    {#if isCreating}
        <div class="mx-4 my-4 flex items-center justify-center rounded px-8 shadow">
            <CaseEditor onClosed={() => (isCreating = false)} onSaved={(e) => {
                isCreating = false
                upsertCase(e)
            }} />
        </div>
    {:else}
        <div class="my-2 flex h-16 flex-row items-center justify-end gap-2 px-4">
            <button class="cursor-pointer" onclick={() => (isCreating = true)}>
                <IconifyIcon class="h-6 w-6" icon="tabler:library-plus" />
            </button>
        </div>
    {/if}

    <div class="md:m-4 md:rounded md:shadow">
        <div
            class="hidden rounded-t md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300"
        >
            <p class="text-md flex-3/12 px-2 py-3 text-left font-bold">Case Name</p>
            <p class="text-md flex-2/12 px-2 py-3 text-left font-bold">Period</p>
            <p class="text-md flex-2/12 px-2 py-3 text-left font-bold text-nowrap">Used Hrs</p>
            <p class="text-md flex-1/12 px-2 py-3 text-left font-bold text-nowrap">Next Billing</p>
            <p class="text-md flex-1/12 px-2 py-3 text-left font-bold text-nowrap">Last Billing</p>
            <p class="text-md flex-auto px-2 py-3 text-left font-bold">&nbsp;</p>
        </div>
        {#each cases as c, i (c.id)}
            <Case
                {...c}
                onSaved={upsertCase}
                onDeleted={() => removeCase(c.id)}
            />
            {#if i < cases.length - 1}
                <div class="mx-2 hidden h-px bg-gray-200 md:block">&nbsp;</div>
            {/if}
        {/each}
    </div>
</main>
