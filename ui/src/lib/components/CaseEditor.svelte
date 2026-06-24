<script lang="ts">
    import Input from '$lib/components/shared/Input.svelte'
    import IconifyIcon from '@iconify/svelte'
    import { CaseServices } from '$lib/services/case.service'
    import Loading from '$lib/components/shared/Loading.svelte'
    import type { CaseData } from '$lib/types'
    import DateTimePicker from '$lib/components/shared/DateTimePicker.svelte'

    type Props = {
        id: string
        name: string
        hrs: number
        billingCycle: number
        startedAt: Date
        endedAt: Date
        settledAt: Date | null
    }
    type Output = {
        onClosed?: () => void
        onSaved?: (props: CaseData) => void
    }
    type PartialProps = Partial<Props> & Output

    let {
        onClosed,
        onSaved,
        id,
        name,
        hrs,
        startedAt,
        endedAt,
        billingCycle,
        settledAt,
    }: PartialProps = $props()
    let _id = $state(id ?? '')
    let _name = $state(name ?? '')
    let _hrs = $state(hrs ?? 0)
    let _startedAt = $state(startedAt ?? new Date())
    let _endedAt = $state(endedAt ?? new Date())
    let _billingCycle = $state(billingCycle ?? 0)
    let isLoading = $state(false)
    let errMsg = $state('')

    function validate() {
        if (_name === '') {
            errMsg = 'Please enter a case name'
            return false
        }

        if ((_hrs ?? 0) <= 0) {
            errMsg = 'Please enter a valid number'
            return false
        }

        return true
    }

    async function _onSaved(e: Event) {
        e.preventDefault()
        errMsg = ''

        if (!validate()) {
            return
        }

        isLoading = true
        const resp = await CaseServices.save({
            ...(_id === '' ? {} : { id: _id }),
            name: _name,
            estimated_minutes: _hrs * 60,
            billing_cycle: _billingCycle,
            started_at: _startedAt,
            ended_at: _endedAt,
        })

        if (resp.error) {
            errMsg = resp.message
        } else {
            onSaved?.({
                id: resp.id,
                name: _name,
                usedMinutes: 0,
                estimatedMinutes: Math.round(_hrs * 60),
                createdAt: new Date(),
                startedAt: _startedAt,
                endedAt: _endedAt,
                pendingLogs: 0,
                billingCycle: _billingCycle,
                settledAt: settledAt ?? null,
            })
        }

        isLoading = false
    }

    function _onClosed(e: Event) {
        e.preventDefault()
        onClosed?.()
    }
</script>

{#if isLoading}
    <Loading />
{/if}

<div class="w-full">
    {#if errMsg}
        <div class="mt-2 w-full text-center text-sm text-red-500">
            {errMsg}
        </div>
    {/if}

    <div
        class="m-4 w-full p-4 md:m-0 md:flex md:min-h-12 md:flex-row md:items-center md:gap-4 md:p-2"
    >
        <div class="my-2 flex-4/12 font-semibold text-nowrap md:my-0 md:font-medium">
            <Input
                label="Case Name"
                name="name"
                type="text"
                variant="outlined"
                bind:value={_name}
            />
        </div>

        <div
            class="my-1 flex flex-2/12 flex-row items-center text-sm text-gray-500 md:my-0 md:flex-col md:text-gray-700"
        >
            <DateTimePicker value={_startedAt} />
            <span>~</span>
            <DateTimePicker value={_endedAt} />
        </div>

        <div class="flex-1/12 text-sm">
            <Input
                label="Hrs"
                name="hrs"
                type="text"
                variant="outlined"
                value={_hrs}
                oninput={(e) => {
                    const n = Number(e.currentTarget.value)
                    if (Number.isNaN(n)) return
                    _hrs = n
                }}
            />
        </div>

        <div class="flex-1/12 text-sm">
            <Input
                label="Billing Cycle (month)"
                name="cycle"
                type="text"
                variant="outlined"
                value={_billingCycle}
                oninput={(e) => {
                    const n = Number(e.currentTarget.value)
                    if (Number.isNaN(n)) return
                    _billingCycle = n
                }}
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
</div>
