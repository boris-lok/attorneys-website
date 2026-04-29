<script lang="ts">
    import DateTimePicker from '$lib/components/DateTimePicker.svelte'
    import AutoCompleteInput from '$lib/components/common/AutoCompleteInput.svelte'
    import { type SimpleUser, UserService } from '$lib/services/user.service'
    import { getSelfId } from '$lib/utils'
    import {
        type WorkLog,
        WorkLogServices
    } from '$lib/services/work_log.service'
    import Textarea from '$lib/components/common/Textarea.svelte'
    import IconifyIcon from '@iconify/svelte'
    import Loading from '$lib/components/common/Loading.svelte'

    type Props = {
        id?: string
        caseId: string
        date?: Date
        description?: string
        duration?: number
        collaboratorIds?: string[]
        hideShare?: boolean
        onClosed?: () => void
        onSaved?: (log: WorkLog) => void
    }

    let {
        id,
        caseId,
        date = new Date(),
        description = '',
        duration = 0,
        collaboratorIds = [],
        hideShare = false,
        onSaved,
        onClosed
    }: Props = $props()
    const everyFifteen = Array.from({ length: 20 }, (_, i) => (i + 1) * 15)
    let users: SimpleUser[] = $state([])
    let loaded = false
    let share = $state(false)
    let req = {
        startedAt: date,
        caseId,
        duration,
        collaboratorIds,
        description
    }
    let errMsg = $state('')
    let isLoading = $state(false)

    function onDateChanged(newDate: Date) {
        req = { ...req, startedAt: newDate }
    }

    function onDurationChanged(newDuration: string) {
        const n = parseInt(newDuration)
        if (isNaN(n)) return
        req = { ...req, duration: n }
    }

    function onDescriptionChanged(e: Event & { currentTarget: HTMLTextAreaElement }) {
        req = { ...req, description: e.currentTarget.value }
    }

    function onCollaboratorIdsChanged(id: string, checked: boolean) {
        let newIds = [...req.collaboratorIds]
        if (checked) {
            newIds = [...newIds, id]
        } else {
            newIds = newIds.filter(e => e !== id)
        }
        req = {
            ...req,
            collaboratorIds: newIds
        }
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
    }

    async function onSave() {
        isLoading = true
        const resp = await WorkLogServices.save({
            ...(id ? { id: id } : {}),
            ...req
        })
        isLoading = false

        if (resp.error) {
            console.error('Error saving work log:', resp.message)
            return
        }

        onSaved?.({
            id: resp.id,
            startedAt: req.startedAt,
            duration: req.duration,
            description: req.description,
            isCollaborative: share,
            collaborators: users
                .filter(e => req.collaboratorIds.includes(e.id))
                .map(e => ({ id: e.id, name: e.nickname }))
        })
    }
</script>

{#if errMsg}
    <div class="px-4">
        <p class="mt-[-1rem] text-sm font-semibold text-red-500">{errMsg}</p>
    </div>
{/if}

{#if isLoading}
    <Loading />
{/if}

<div class="flex flex-col gap-4">

    <DateTimePicker date={date} onChanged={onDateChanged} />

    <AutoCompleteInput name="duration" options={async () => {
           return everyFifteen.map(m => ({
               key: m.toString(),
               value: m.toString(),
           }))
       }} value={duration.toString()} label="Duration" onBlur={onDurationChanged}
                       onSelect={e => onDurationChanged(e.key)} />

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
            <div class="flex gap-4 flex-row flex-wrap py-2">
                {#each users as user (user.id)}
                    <label for={user.id} class="cursor-pointer text-md font-medium">
                        <input type="checkbox" id={user.id} value={user.id} class="mr-2"
                               checked={collaboratorIds.includes(user.id)} onchange={e => {
                               onCollaboratorIdsChanged(user.id, e.currentTarget.checked)
                           }} />{ user.nickname}
                    </label>
                {/each}
            </div>
        {/if}
    {/if}

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

