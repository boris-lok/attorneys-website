<script lang="ts">
    import DateTimePicker from '$lib/components/DateTimePicker.svelte'
    import { type SimpleUser, UserService } from '$lib/services/user.service'
    import { getSelfId } from '$lib/utils'
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
        _e.setSeconds(0)
        _e.setMilliseconds(0)
        _s.setSeconds(0)
        _s.setMilliseconds(0)

        return ((_e.getTime() - _s.getTime()) / 1000 / 60 )
    })
    let creator: SimpleUser | undefined;

    function onDateChanged(key: 'startedAt' | 'endedAt', newDate: Date) {
        console.log('onDateChanged', key, newDate)
        if (key === 'startedAt') {
            _startedAt = newDate
        } else {
            _endedAt = newDate
        }
    }

    function onDescriptionChanged(e: Event & { currentTarget: HTMLTextAreaElement }) {
        _description = e.currentTarget.value
    }

    function onCollaboratorIdsChanged(id: string, checked: boolean) {
        let newIds = [...collaboratorIds]
        if (checked) {
            newIds = [...newIds, id]
        } else {
            newIds = newIds.filter(e => e !== id)
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
        const selfId = getSelfId()
        creator = resp.users.find(e => e.id === selfId)
        console.log('creator', creator)

        users = resp
            .users
            .filter(e => e.id !== selfId)
            .filter(e => e.roles.includes('Lawyer'))
    }

    async function onShareChanged(e: Event) {
        let elem = e.target as HTMLInputElement
        if (elem.checked) {
            await fetchUsers()
        }

        share = elem.checked
        if (share) {
            _collaboratorIds = users.map(e => e.id)
        } else {
            _collaboratorIds = []
        }
    }

    async function onSave() {

        const validate = () => {
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

        let collaborators: Collaborator[] =
            users
                .filter(e => _collaboratorIds.includes(e.id))
                .map(e => ({ parentId: resp.id, userId: e.id, name: e.nickname, status: 'pending' }))

        onSaved?.({
            id: resp.id,
            startedAt: _startedAt,
            endedAt: _endedAt,
            duration: duration,
            description: _description,
            isCollaborative: share,
            collaborators: collaborators,
            user: {
                id: getSelfId(),
                name: creator?.nickname ?? 'Unknown'
            }
        })
    }
</script>

{#if errMsg}
    <div class="px-4">
        <p class="mt-1 text-sm font-semibold text-red-500 text-center">{errMsg}</p>
    </div>
{/if}

{#if isLoading}
    <Loading />
{/if}

<div class="flex flex-col gap-4">

    <div class="flex flex-row items-center justify-between md:justify-normal md:gap-4">
        <div class="flex flex-col md:flex-row md:gap-2 md:items-center gap-1">
            <span class="text-sm font-semibold">Working Time: </span>
            <DateTimePicker date={startedAt} onChanged={e => onDateChanged('startedAt', e) } />
            <span class="text-center"> ~ </span>
            <DateTimePicker date={endedAt} onChanged={e => onDateChanged('endedAt', e) } />
        </div>

        <span class="h-fit"> ({duration} min)</span>
    </div>

    <div>
        <Textarea label="Description" name="description" value={description} onInput={onDescriptionChanged}
                  height="h-36" />
    </div>

    {#if !hideShare}
        <label class="inline-flex items-center cursor-pointer">
            <input type="checkbox" value="" class="sr-only peer" onclick={onShareChanged}>
            <div
                class="relative w-9 h-5 bg-gray-500 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-blue-500"></div>
            <span class="select-none ms-3 text-sm font-medium text-heading">Share</span>
        </label>


        {#if share}
            <div class="flex gap-4 flex-row flex-wrap px-2 rounded bg-gray-100 mx-2 my-1 py-2">
                {#each users as user (user.id)}
                    <label for={user.id} class="cursor-pointer text-md font-medium">
                        <input type="checkbox" id={user.id} value={user.id} class="mr-2"
                               checked={_collaboratorIds.includes(user.id)} onchange={e => {
                               onCollaboratorIdsChanged(user.id, e.currentTarget.checked)
                           }} />{ user.nickname}
                    </label>
                {/each}
            </div>
        {/if}
    {/if}

    <div class="flex h-fit flex-row gap-0.5 justify-center">
        <button class="cursor-pointer md:m-2" onclick={onSave}>
            <IconifyIcon
                class=" text-green-500 h-6 w-6"
                icon="charm:square-tick"
            />
        </button>
        <button class="cursor-pointer md:m-2" onclick={onClosed}>
            <IconifyIcon
                class="text-red-500 h-6 w-6"
                icon="line-md:close-square"
            />
        </button>
    </div>

</div>

