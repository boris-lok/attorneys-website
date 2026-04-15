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
        <div>
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

    <div class="px-2">
        <div
            class="mt-2 flex w-full flex-row items-center justify-between gap-2 rounded-t border-t border-r border-l border-gray-300 bg-gray-300 px-4 py-2"
        >
            <p class="flex-4/6 text-sm font-bold">Case</p>
            <p class="flex-1/6 text-sm font-bold">Duration</p>
            <p>&nbsp;</p>
        </div>
        <div class="border-r border-b border-l border-gray-300">
            {#each cases as c (c.id)}
                <Case
                    id={c.id}
                    name={c.name}
                    estimatedMinutes={c.estimatedMinutes}
                />
            {/each}
        </div>
    </div>
</main>
