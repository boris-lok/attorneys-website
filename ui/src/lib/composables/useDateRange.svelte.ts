export function useDateRange(daysBack: number = 90) {
    function startedAt(d: Date) {
        return new Date(d.getFullYear(), d.getMonth(), d.getDate(), 0, 0, 0, 0)
    }

    function endedAt(d: Date) {
        return new Date(d.getFullYear(), d.getMonth(), d.getDate(), 23, 59, 59, 999)
    }

    let _startedAt = $state(startedAt(new Date(Date.now() - daysBack * 24 * 60 * 60 * 1000)))
    let _endedAt = $state(endedAt(new Date()))

    return {
        get startedAt() {
            return _startedAt
        },
        get endedAt() {
            return _endedAt
        },
        set: (t: 'startedAt' | 'endedAt', d: Date) => {
            if (t === 'startedAt') _startedAt = startedAt(d)
            if (t === 'endedAt') _endedAt = endedAt(d)
        },
    }
}
