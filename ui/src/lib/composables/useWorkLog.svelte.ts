import type { PayLoad } from '$lib/utils'
import type { PendingWorkLog, WorkLog } from '$lib/types'
import { WorkLogServices } from '$lib/services/workLog.service'

export function useWorkLog(caseId: string, payLoad: PayLoad) {
    let logs = $state<WorkLog[]>([])
    const pendingWorkLogs = $derived.by(() => {
        return logs
            .filter((log) =>
                log.collaborators.some((c) => c.status === 'pending' && c.userId === payLoad.sub)
            )
            .map(
                (log) =>
                    ({
                        id: log.id,
                        startedAt: log.startedAt,
                        endedAt: log.endedAt,
                        duration: log.duration,
                        description: log.description,
                        user: { id: payLoad.sub, name: payLoad.nickname },
                    }) satisfies PendingWorkLog
            )
    })
    let isLoading = $state(false)
    let errMsg = $state('')

    async function fetch(startedAt: Date, endedAt: Date) {
        isLoading = true
        errMsg = ''
        const resp = await WorkLogServices.list(window.fetch, caseId, startedAt, endedAt)
        isLoading = false

        if (resp.error) {
            errMsg = resp.message
            return
        }

        logs = resp.logs
    }

    function editStatus(id: string, status: 'accepted' | 'pending' | 'rejected') {
        logs = logs.filter((log) =>
            log.id !== id
                ? log
                : {
                      ...log,
                      collaborators: log.collaborators.map((c) =>
                          c.userId === payLoad.sub ? { ...c, status } : c
                      ),
                  }
        )
    }

    function upsert(log: WorkLog) {
        const exist = logs.some((e) => e.id === log.id)
        logs = exist ? logs.map((e) => (e.id === log.id ? log : e)) : [log, ...logs]
    }

    function remove(id: string) {
        logs = logs.filter((log) => log.id !== id)
    }

    return {
        get logs() {
            return logs
        },
        get pendingLogs() {
            return pendingWorkLogs
        },
        get isLoading() {
            return isLoading
        },
        get errMsg() {
            return errMsg
        },
        fetch,
        editStatus,
        upsert,
        remove,
    }
}
