<!-- svelte-ignore state_referenced_locally -->
<script lang="ts">
    import DateTimePicker from '$lib/components/shared/DateTimePicker.svelte'
    import { UserService } from '$lib/services/user.service'
    import Textarea from '$lib/components/shared/Textarea.svelte'
    import IconifyIcon from '@iconify/svelte'
    import type { SimpleUser, WorkLog } from '$lib/types'
    import { toast } from '$lib/stores/toast.svelte'
    import { WorkLogServices } from '$lib/services/workLog.service'

    // Constants
    const FIFTEEN_MINUTES_MS = 15 * 60 * 1000

    const VALIDATION_MESSAGE = {
        missingStartedAt: 'Please select a start date',
        missingEndedAt: 'Please select a end data',
        missingDescription: 'Please enter a description',
        invalidDuration: 'Duration must be at least 15 minutes',
        missingCollaborator: 'Please select at least one collaborator',
    } as const

    // Types
    type Props = {
        id?: string
        selfId: string
        selfName: string
        caseId: string
        startedAt?: Date
        endedAt?: Date
        description?: string
        collaboratorIds?: string[]
        hideShare?: boolean
        onClosed?: () => void
        onSaved?: (log: WorkLog) => void
    }

    // Props
    let {
        id,
        caseId,
        selfId,
        selfName,
        startedAt = new Date(),
        endedAt = new Date(Date.now() + FIFTEEN_MINUTES_MS),
        description = '',
        collaboratorIds = [],
        hideShare = false,
        onSaved,
        onClosed,
    }: Props = $props()

    // State
    let lawyers: SimpleUser[] = $state([])
    let isLawyerLoaded = false

    let share = $state(false)
    let isLoading = $state(false)
    let errMsg = $state('')

    let _startedAt = $state(startedAt)
    let _endedAt = $state(endedAt)
    let _description = $state(description)
    let _collaboratorIds = $state(collaboratorIds)

    // Derived
    let duration = $derived(computeDuration(_startedAt, _endedAt))

    // Pure Helpers
    function computeDuration(start: Date, end: Date) {
        const s = new Date(start)
        const e = new Date(end)
        // ignore time part
        e.setSeconds(0, 0)
        s.setSeconds(0, 0)

        return (e.getTime() - s.getTime()) / 1000 / 60
    }

    function buildCollaborators(ids: string[], users: SimpleUser[], parentId: string) {
        return users
            .filter((u) => ids.includes(u.id))
            .map((e) => {
                return {
                    parentId,
                    userId: e.id,
                    name: e.nickname,
                    status: 'pending' as const,
                }
            })
    }

    // Data fetching
    async function fetchLawyers() {
        if (isLawyerLoaded) return
        isLawyerLoaded = true
        const resp = await UserService.list()

        if (resp.error) {
            isLawyerLoaded = false
            toast.show(`Error loading users: ${resp.message}`)
            return
        }

        lawyers = resp.users
            .filter((e) => e.id !== selfId)
            .filter((e) => e.roles.includes('Lawyer'))
    }

    // Event Handlers
    function onStartedDateChanged(date: Date | null) {
        if (!date) return
        _startedAt = date
        _endedAt = new Date(date.getTime() + FIFTEEN_MINUTES_MS)
    }

    async function onShareToggle() {
        _collaboratorIds = []

        if (share) {
            await fetchLawyers()
            _collaboratorIds = lawyers.map((e) => e.id)
        }
    }

    function onCollaboratorIdsChanged(id: string, checked: boolean) {
        let newIds = [..._collaboratorIds]
        if (checked) {
            newIds = [...newIds, id]
        } else {
            newIds = newIds.filter((e) => e !== id)
        }
        _collaboratorIds = newIds
    }

    function validate() {
        if (!_startedAt) {
            errMsg = VALIDATION_MESSAGE.missingStartedAt
            return false
        }
        if (!_endedAt) {
            errMsg = VALIDATION_MESSAGE.missingEndedAt
            return false
        }
        if (!_description) {
            errMsg = VALIDATION_MESSAGE.missingDescription
            return false
        }
        if (duration <= 0) {
            errMsg = VALIDATION_MESSAGE.invalidDuration
            return false
        }
        if (share && _collaboratorIds.length === 0) {
            errMsg = VALIDATION_MESSAGE.missingCollaborator
            return false
        }

        return true
    }

    async function onSave() {
        errMsg = ''
        if (!validate()) return

        isLoading = true
        const resp = await WorkLogServices.save(window.fetch, {
            ...(id ? { id: id } : {}),
            case_id: caseId,
            collaborator_ids: _collaboratorIds,
            description: _description,
            duration: duration,
            started_at: _startedAt,
        })
        isLoading = false

        if (resp.error) {
            toast.show(`Error saving work log: got an error: ${resp.message}`, 'error')
            return
        }

        onSaved?.({
            id: resp.id,
            startedAt: _startedAt,
            endedAt: _endedAt,
            duration: duration,
            description: _description,
            isCollaborative: share,
            collaborators: buildCollaborators(_collaboratorIds, lawyers, resp.id),
            user: {
                id: selfId,
                name: selfName,
            },
        })
    }
</script>

{#if errMsg}
    <div class="px-4">
        <p class="mt-1 text-center text-sm font-semibold text-red-500">
            {errMsg}
        </p>
    </div>
{/if}

<div class="flex flex-col gap-4">
    <div class="flex flex-row items-center justify-between md:justify-normal md:gap-4">
        <div class="flex flex-col gap-1 sm:flex-row sm:items-center md:items-center md:gap-2">
            <span class="h-fit text-sm font-semibold">Working Time: </span>
            <DateTimePicker value={_startedAt} onchange={onStartedDateChanged} showTime />
            <span class="hidden text-center sm:block"> ~ </span>
            <DateTimePicker value={_endedAt} showTime />
        </div>

        <span class="h-fit"> ({duration} min)</span>
    </div>

    <div>
        <Textarea label="Description" name="description" bind:value={_description} height="h-36" />
    </div>

    {#if !hideShare}
        <label class="inline-flex cursor-pointer items-center">
            <input
                type="checkbox"
                bind:checked={share}
                class="peer sr-only"
                onchange={onShareToggle}
            />
            <div
                class="peer relative h-5 w-9 rounded-full bg-gray-500 peer-checked:bg-blue-500 peer-focus:outline-none after:absolute after:inset-s-0.5 after:top-0.5 after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full"
            ></div>
            <span class="text-heading ms-3 text-sm font-medium select-none">Share</span>
        </label>

        {#if share}
            <div class="mx-2 my-1 flex flex-row flex-wrap gap-4 rounded bg-gray-100 px-2 py-2">
                {#each lawyers as user (user.id)}
                    <label for={user.id} class="text-md cursor-pointer font-medium">
                        <input
                            type="checkbox"
                            bind:group={_collaboratorIds}
                            id={user.id}
                            value={user.id}
                            class="mr-2"
                            onchange={(e) => {
                                onCollaboratorIdsChanged(user.id, e.currentTarget.checked)
                            }}
                        />{user.nickname}
                    </label>
                {/each}
            </div>
        {/if}
    {/if}

    <div class="flex h-fit flex-row justify-center gap-0.5">
        {#if isLoading}
            <div>
                <IconifyIcon
                    class="mr-6 h-6 w-6 text-blue-500"
                    icon="svg-spinners:90-ring-with-bg"
                />
            </div>
        {:else}
            <button class="cursor-pointer md:m-2" onclick={onSave}>
                <IconifyIcon
                    class=" h-6 w-6 text-green-500"
                    icon="line-md:circle-to-confirm-circle-transition"
                />
            </button>
            <button class="cursor-pointer md:m-2" onclick={onClosed}>
                <IconifyIcon class="h-6 w-6 text-red-500" icon="line-md:close-circle" />
            </button>
        {/if}
    </div>
</div>
