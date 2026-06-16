<script lang="ts">
    import DateTimePicker from '$lib/components/common/DateTimePicker.svelte'
    import { type SimpleUser, UserService } from '$lib/services/user.service'
    import {
        type Collaborator,
        type WorkLog,
        WorkLogServices
    } from '$lib/services/work_log.service'
    import Textarea from '$lib/components/common/Textarea.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'

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

    let {
        id,
        caseId,
        selfId,
        selfName,
        startedAt = new Date(),
        endedAt = new Date(),
        description = '',
        collaboratorIds = [],
        hideShare = false,
        onSaved,
        onClosed
    }: Props = $props()
    let users: SimpleUser[] = $state([])
    let loaded = false
    let share = $state(false)
    let _startedAt = $state(startedAt)
    let _endedAt = $state(endedAt)
    let _description = $state(description)
    let _collaboratorIds = $state(collaboratorIds)
    let errMsg = $state('')
    let isLoading = $state(false)
    let duration = $derived.by(() => {
        const _e = new Date(_endedAt)
        const _s = new Date(_startedAt)

        // ignore time part
        _e.setSeconds(0, 0)
        _s.setSeconds(0, 0)

        return (_e.getTime() - _s.getTime()) / 1000 / 60
    })
    let fifteenMinutes = 15 * 60 * 1000

    function onCollaboratorIdsChanged(id: string, checked: boolean) {
        let newIds = [..._collaboratorIds]
        if (checked) {
            newIds = [...newIds, id]
        } else {
            newIds = newIds.filter((e) => e !== id)
        }
        _collaboratorIds = newIds
    }

    async function fetchUsers() {
        if (loaded) return
        loaded = true
        const resp = await UserService.list()

        if (resp.error) {
            loaded = false
            console.error('Error loading users:', resp.message)
            return
        }

        users = resp.users.filter((e) => e.id !== selfId).filter((e) => e.roles.includes('Lawyer'))
    }

    async function onShareChanged() {
        if (share) {
            await fetchUsers()
            _collaboratorIds = users.map((e) => e.id)
        } else {
            _collaboratorIds = []
        }
    }

    function validate() {
        if (!_startedAt) {
            errMsg = 'Please select start time'
            return false
        }
        if (!_endedAt) {
            errMsg = 'Please select end time'
            return false
        }
        if (!_description) {
            errMsg = 'Please enter description'
            return false
        }
        if (duration <= 0) {
            errMsg = 'Ended date must be later than start date'
            return false
        }

        return true
    }

    async function onSave() {
        if (!validate()) {
            return
        }

        isLoading = true
        const resp = await WorkLogServices.save({
            ...(id ? { id: id } : {}),
            caseId: caseId,
            collaboratorIds: _collaboratorIds,
            description: _description,
            duration: duration,
            startedAt: _startedAt
        })
        isLoading = false

        if (resp.error) {
            console.error('Error saving work log:', resp.message)
            return
        }

        let collaborators: Collaborator[] = users
            .filter((e) => _collaboratorIds.includes(e.id))
            .map((e) => ({
                parentId: resp.id,
                userId: e.id,
                name: e.nickname,
                status: 'pending'
            }))

        onSaved?.({
            id: resp.id,
            startedAt: _startedAt,
            endedAt: _endedAt,
            duration: duration,
            description: _description,
            isCollaborative: share,
            collaborators: collaborators,
            user: {
                id: selfId,
                name: selfName
            }
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

{#if isLoading}
    <Loading />
{/if}

<div class="flex flex-col gap-4">
    <div class="flex flex-row items-center justify-between md:justify-normal md:gap-4">
        <div class="flex flex-col gap-1 md:flex-row md:items-center md:gap-2">
            <span class="text-sm font-semibold">Working Time: </span>
            <DateTimePicker date={_startedAt} onChanged={(e) => {
                _startedAt = e
                _endedAt = new Date(_startedAt.getTime() + fifteenMinutes)
            }} />
            <span class="text-center"> ~ </span>
            <DateTimePicker date={_endedAt} onChanged={(e) => _endedAt = e} />
        </div>

        <span class="h-fit"> ({duration} min)</span>
    </div>

    <div>
        <Textarea
            label="Description"
            name="description"
            bind:value={_description}
            height="h-36"
        />
    </div>

    {#if !hideShare}
        <label class="inline-flex cursor-pointer items-center">
            <input type="checkbox" bind:checked={share} class="peer sr-only" onchange={onShareChanged} />
            <div
                class="peer relative h-5 w-9 rounded-full bg-gray-500 peer-checked:bg-blue-500 peer-focus:outline-none after:absolute after:start-[2px] after:top-[2px] after:h-4 after:w-4 after:rounded-full after:bg-white after:transition-all after:content-[''] peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full"
            ></div>
            <span class="text-heading ms-3 text-sm font-medium select-none">Share</span>
        </label>

        {#if share}
            <div class="mx-2 my-1 flex flex-row flex-wrap gap-4 rounded bg-gray-100 px-2 py-2">
                {#each users as user (user.id)}
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
        <button class="cursor-pointer md:m-2" onclick={onSave}>
            <IconifyIcon
                class=" h-6 w-6 text-green-500"
                icon="line-md:circle-to-confirm-circle-transition"
            />
        </button>
        <button class="cursor-pointer md:m-2" onclick={onClosed}>
            <IconifyIcon class="h-6 w-6 text-red-500" icon="line-md:close-circle" />
        </button>
    </div>
</div>
