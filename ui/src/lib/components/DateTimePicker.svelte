<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'

    type Props = {
        date?: Date
        onChanged?: (date: Date) => void
        dateOnly?: boolean
    }

    let { date = new Date(), onChanged, dateOnly = false }: Props = $props()

    type Draft = {
        year: string
        month: string
        day: string
        hour: string
        minute: string
    }

    function toDraft(d: Date): Draft {
        return {
            year: String(d.getFullYear()),
            month: String(d.getMonth() + 1),
            day: String(d.getDate()),
            hour: String(d.getHours()),
            minute: String(d.getMinutes()),
        }
    }

    let lastSource = $state(date)
    let draft = $state<Draft>(toDraft(date))

    function tryCommit(): boolean {
        const y = Number(draft.year)
        const m = Number(draft.month)
        const d = Number(draft.day)
        const h = dateOnly ? 0 : Number(draft.hour)
        const min = dateOnly ? 0 : Number(draft.minute)

        const requiredEmpty =
            draft.year === '' ||
            draft.month === '' ||
            draft.day === '' ||
            (!dateOnly && (draft.hour === '' || draft.minute === ''))

        if (requiredEmpty || [y, m, d, h, min].some(Number.isNaN)) return false

        // Range checks
        if (y < 1000 || y > 9999) return false
        if (m < 1 || m > 12) return false
        if (d < 1 || d > 31) return false
        if (!dateOnly && (h < 0 || h > 23 || min < 0 || min > 59)) return false

        const newDate = new Date(y, m - 1, d, h, min)
        newDate.setFullYear(y) // guard against 2-digit-year legacy

        // Detect rollover (e.g. Feb 30 → Mar 2)
        if (
            newDate.getFullYear() !== y ||
            newDate.getMonth() !== m - 1 ||
            newDate.getDate() !== d
        ) {
            return false
        }

        if (newDate.getTime() === date.getTime()) return true // no-op

        lastSource = newDate
        onChanged?.(newDate)
        return true
    }

    function commitOrRevert() {
        if (!tryCommit()) {
            draft = toDraft(date)
        }
    }

    function updateField(key: keyof Draft, value: string) {
        if (value !== '' && !/^\d+$/.test(value)) return
        draft[key] = value
    }

    function onKeyDown(e: KeyboardEvent) {
        if (e.key === 'Enter') {
            ;(e.currentTarget as HTMLInputElement).blur()
        }
    }

    // sync from parent only when its date actually changes
    $effect(() => {
        if (date.getTime() !== lastSource.getTime()) {
            lastSource = date
            draft = toDraft(date)
        }
    })
</script>

{#snippet field(key: keyof Draft, width: string, maxlength: number)}
    <div class={width}>
        <Input
            name={key}
            type="text"
            inputmode="numeric"
            {maxlength}
            variant="outlined"
            aria-label={key}
            value={draft[key]}
            oninput={(e) => updateField(key, e.currentTarget.value)}
            onblur={commitOrRevert}
            onkeydown={onKeyDown}
        />
    </div>
{/snippet}

<div class="flex flex-wrap items-center gap-1">
    {@render field('year', 'w-16', 4)}
    <span class="text-gray-500">/</span>
    {@render field('month', 'w-10', 2)}
    <span class="text-gray-500">/</span>
    {@render field('day', 'w-10', 2)}

    {#if !dateOnly}
        <span class="w-2"></span>
        {@render field('hour', 'w-10', 2)}
        <span class="text-gray-500">:</span>
        {@render field('minute', 'w-10', 2)}
    {/if}
</div>