<script lang="ts">
    import type { CaseData } from '$lib/types'
    import Case from '$lib/components/Case.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'
    import CaseEditor from '$lib/components/CaseEditor.svelte'
    import type { PageProps } from './$types'

    let { data }: PageProps = $props()

    let isCreating = $state(false)
    let isLoaded = $state(false)
    let cases: CaseData[] = $state([])
    let errMsg = $state('')

    function upsertCase(c: CaseData) {
        cases = [
            c,
            ...cases.filter((e) => e.id !== c.id),
        ]
    }

    $effect(() => {
        data.cases
            .then(resp => {
                if (resp.error) errMsg = resp.message
                else cases = resp.cases
            })
            .finally(() => isLoaded = true)
    })

</script>

{#if !isLoaded}
    <Loading />
{/if}

{#if errMsg}
    <div class="mt-2 text-sm text-red-500 w-full text-center">
        {errMsg}
    </div>
{/if}

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
            <p class="text-md flex-1/12 px-2 py-3 text-left font-bold text-nowrap">Next Billing At</p>
            <p class="text-md flex-1/12 px-2 py-3 text-left font-bold text-nowrap">Last Settled At</p>
            <p class="text-md flex-auto px-2 py-3 text-left font-bold">&nbsp;</p>
        </div>
        {#each cases as c, i (c.id)}
            <Case
                {...c}
                onSaved={(updated) => {
                    cases = cases.map((c) =>c.id === updated.id ? updated : c )
                }}
                onDeleted={() => (cases = cases.filter((e) => e.id !== c.id))}
            />
            {#if i < cases.length - 1}
                <div class="mx-2 hidden h-px bg-gray-200 md:block">&nbsp;</div>
            {/if}
        {/each}
    </div>
</main>
