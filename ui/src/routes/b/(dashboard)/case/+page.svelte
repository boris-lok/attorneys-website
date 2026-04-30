<script lang="ts">
    import type { CaseData } from '$lib/types'
    import { CaseServices } from '$lib/services/case.service'
    import Case from '$lib/components/Case.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'
    import CaseEditor from '$lib/components/CaseEditor.svelte'

    let isLoading = $state(false)
    let isCreated = $state(false)
    let cases: CaseData[] = $state([])

    async function fetchCases(): Promise<CaseData[]> {
        isLoading = true
        try {
            const resp = await CaseServices.list()
            if (resp.error) {
                console.error(resp.message)
                return []
            }

            return resp.cases
        } catch (e) {
            console.error(e)
            return []
        } finally {
            isLoading = false
        }
    }

    function appendCase(c: CaseData) {
        cases = [c, ...cases]
    }

    $effect(() => {
        const load = async () => {
            cases = await fetchCases()
        }

        load()
    })
</script>

{#if isLoading}
    <Loading />
{/if}

<main>
    {#if isCreated}
        <div class="flex items-center justify-center px-8 my-4 shadow rounded mx-4">
            <CaseEditor
                onClosed={() => (isCreated = false)}
                onSaved={appendCase}
            />
        </div>
    {:else}
        <div
            class="my-2 flex h-16 flex-row items-center justify-end gap-2 px-4"
        >
            <button class="cursor-pointer" onclick={() => (isCreated = true)}>
                <IconifyIcon
                    class="h-4 w-4 md:h-6 md:w-6"
                    icon="solar:add-folder-broken"
                />
            </button>
        </div>
    {/if}

    <div class="md:rounded md:shadow md:m-4">
        <div class="hidden md:flex md:w-full md:border-b md:border-b-gray-200 md:bg-gray-300 rounded-t">
            <p class="flex-6/12  text-left px-2 text-md py-3 font-bold">Case Name</p>
            <p class="flex-2/12 text-left px-2 text-md py-3 font-bold">Period</p>
            <p class="flex-2/12 text-left px-2 text-md py-3 font-bold">Used Hrs</p>
            <p class="flex-auto text-left px-2 text-md py-3 font-bold">&nbsp;</p>
        </div>
        {#each cases as c, i (c.id)}
            <Case {...c} onSaved={(e) => (cases[i] = e)} />
            {#if i < cases.length - 1}
                <div class="hidden md:block bg-gray-200 mx-2 h-[1px]">&nbsp;</div>
            {/if}
        {/each}
    </div>
</main>
