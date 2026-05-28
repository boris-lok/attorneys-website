<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { CaseServices } from '$lib/services/case.service'
    import Loading from '$lib/components/common/Loading.svelte'
    import type { CaseData } from '$lib/types'
    import DateTimePicker from '$lib/components/DateTimePicker.svelte'

    type Props = {
        id: string
        name: string
        hrs: number
        startedAt: Date
        endedAt: Date
    }
    type Output = {
        onClosed?: () => void
        onSaved?: (props: CaseData) => void
    }
    type PartialProps = Partial<Props> & Output

    let { onClosed, onSaved, ...rest }: PartialProps = $props()
    const now = new Date()
    let copiedData: Props = $state({
        id: rest.id ?? '',
        name: rest.name ?? '',
        hrs: rest.hrs ?? 0,
        startedAt:
            rest.startedAt ?? new Date(now.getFullYear(), now.getMonth(), now.getDate(), 0, 0, 0),
        endedAt:
            rest.endedAt ??
            new Date(now.getFullYear(), now.getMonth(), now.getDate() + 1, 23, 59, 59),
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

    function onDateChanged(key: 'startedAt' | 'endedAt', newValue: Date) {
        console.log(key, newValue, 'onDateChanged')
        if (key === 'startedAt') {
            copiedData = { ...copiedData, startedAt: newValue }
        } else if (key === 'endedAt') {
            copiedData = { ...copiedData, endedAt: newValue }
        }
    }

    async function _onSaved(e: Event) {
        e.preventDefault()
        e.stopPropagation()
        errMsg = ''
        console.log(copiedData)
        const validate = () => {
            if (copiedData.name === '') {
                errMsg = 'Please enter a case name'
                return false
            }
            if ((copiedData.hrs ?? 0) <= 0) {
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
            name: copiedData.name!,
            estimated_minutes: copiedData.hrs! * 60,
            started_at: copiedData.startedAt,
            ended_at: copiedData.endedAt,
        })

        if (resp.error) {
            errMsg = resp.message
        } else {
            onClosed?.()
            onSaved?.({
                id: resp.id,
                name: copiedData.name!,
                usedMinutes: 0,
                estimatedMinutes: copiedData.hrs! * 60,
                createdAt: now,
                startedAt: copiedData.startedAt,
                endedAt: copiedData.endedAt,
                pendingLogs: 0,
            })
        }

        isLoading = false
    }

    function _onClosed(e: Event) {
        e.preventDefault()
        e.stopPropagation()
        onClosed?.()
    }
</script>

{#if isLoading}
    <Loading />
{/if}

<div
    class="m-4 w-full rounded p-4 shadow md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:rounded-none md:p-2 md:shadow-none"
>
    <div class="my-2 flex-6/12 font-semibold text-nowrap md:my-0 md:font-medium">
        <Input
            value={copiedData.name}
            label="Case Name"
            name="name"
            type="text"
            variant="outlined"
            onInput={(e) => onPropsChanged('name', e.currentTarget.value)}
        />
    </div>

    <div
        class="my-1 flex flex-2/12 flex-row items-center text-sm text-gray-500 md:my-0 md:flex-col md:text-gray-700"
    >
        <DateTimePicker
            date={copiedData.startedAt}
            onChanged={(e) => onDateChanged('startedAt', e)}
            dateOnly={true}
        />
        <span>~</span>
        <DateTimePicker
            date={copiedData.endedAt}
            onChanged={(e) => onDateChanged('endedAt', e)}
            dateOnly={true}
        />
    </div>

    <div class="flex-2/12 text-sm">
        <Input
            value={copiedData.hrs}
            label="Hrs"
            name="hrs"
            type="text"
            variant="outlined"
            onInput={(e) => onPropsChanged('hrs', e.currentTarget.value)}
        />
    </div>

    <div class="flex h-fit flex-auto flex-row justify-center gap-2 md:justify-end">
        <button class="cursor-pointer md:m-0.5" onclick={_onSaved}>
            <IconifyIcon
                class="h-6 w-6 text-green-500"
                icon="line-md:circle-to-confirm-circle-transition"
            />
        </button>
        <button class="cursor-pointer md:m-0.5" onclick={_onClosed}>
            <IconifyIcon class="h-6 w-6 text-red-500" icon="line-md:close-circle" />
        </button>
    </div>
</div>
