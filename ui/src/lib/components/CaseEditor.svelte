<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { CaseServices } from '$lib/services/case.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import type { CaseData } from '$lib/types'

    type Props = {
        id: string
        name: string
        hrs: number
        onClosed?: () => void
        onSaved?: (props: CaseData) => void
    }
    type PartialProps = Partial<Props>

    let { id, name, hrs, onClosed, onSaved }: PartialProps = $props()
    let copiedData: Props = $state({
        id: id ?? '',
        name: name ?? '',
        hrs: hrs ?? 0,
    })
    let errMsg = $state('')
    let isLoading = $state(false)

    function onPropsChanged(key: keyof Props, newValue: string) {
        if (key === 'name') {
            copiedData = { ...copiedData, name: newValue }
        } else if (key === 'hrs') {
            const n = Number(newValue)
            if (isNaN(n)) {
                errMsg = 'Please enter a valid number'
                return
            }
            copiedData = { ...copiedData, hrs: n }
        }
    }

    async function onSave() {
        errMsg = ''
        const validate = () => {
            if (copiedData.name === '') {
                errMsg = 'Please enter a name'
                return false
            }
            if (copiedData.hrs <= 0) {
                errMsg = 'Please enter a valid number'
                return false
            }
            return true
        }

        if (!validate()) {
            return
        }

        isLoading = true
        const resp = await CaseServices.save({
            ...(copiedData.id === '' ? {} : { id: copiedData.id }),
            name: copiedData.name,
            estimated_minutes: copiedData.hrs * 60,
        })

        if (resp.error) {
            errMsg = resp.message
        } else {
            name = copiedData.name
            hrs = copiedData.hrs
            onClosed?.()
            onSaved?.({
                id: resp.id,
                name: name,
                estimatedMinutes: hrs * 60,
                createdAt: new Date(),
                createdAtString: '',
            })
        }

        isLoading = false
    }
</script>

{#if isLoading}
    <Loading />
{/if}

<div
    class="mt-2 flex h-16 w-full flex-row items-center justify-between gap-2 px-4"
>
    <div class="flex-4/6">
        <Input
            value={copiedData.name}
            name="name"
            type="text"
            variant="outlined"
            onInput={(e) => onPropsChanged('name', e.currentTarget.value)}
        />
    </div>

    <div class="flex-1/6">
        <Input
            value={copiedData.hrs}
            name="hrs"
            type="text"
            variant="outlined"
            onInput={(e) => onPropsChanged('hrs', e.currentTarget.value)}
        />
    </div>

    <div class="flex h-fit flex-row gap-0.5">
        <button class="cursor-pointer md:m-2" onclick={onSave}>
            <IconifyIcon
                class="h-4 w-4 text-green-500 md:h-6 md:w-6"
                icon="charm:square-tick"
            />
        </button>
        <button class="cursor-pointer md:m-2" onclick={onClosed}>
            <IconifyIcon
                class="h-4 w-4 text-red-500 md:h-6 md:w-6"
                icon="line-md:close-square"
            />
        </button>
    </div>
</div>

{#if errMsg}
    <div class="px-4">
        <p class="mt-[-1rem] text-sm font-semibold text-red-500">{errMsg}</p>
    </div>
{/if}
