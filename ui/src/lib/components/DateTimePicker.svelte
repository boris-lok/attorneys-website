<script lang="ts">
    import AutoCompleteInput from '$lib/components/common/AutoCompleteInput.svelte'

    type Props = {
        date?: Date
        onChanged?: (date: Date) => void
        dateOnly?: boolean
    }
    let { date = new Date(), onChanged, dateOnly }: Props = $props()

    let year = $state(date.getFullYear())
    let month = $state(date.getMonth() + 1)
    let day = $state(date.getDate())
    let hour = $state(date.getHours())
    let minute = $state(date.getMinutes())

    const years = Array.from({ length: 10 }, (_, i) => year + i)
    const months = Array.from({ length: 12 }, (_, i) => i + 1)
    const hours = Array.from({ length: 24 }, (_, i) => i)
    const minutes = Array.from({ length: 60 }, (_, i) => i)

    const days = $derived(new Date(year, month, 0).getDate())
    const dayOptions = $derived(Array.from({ length: days }, (_, i) => i + 1))

    function onChange(
        key: 'year' | 'month' | 'day' | 'hour' | 'minute',
        value: string,
    ) {
        switch (key) {
            case 'year':
                year = parseInt(value)
                break
            case 'month':
                month = parseInt(value)
                break
            case 'day':
                day = parseInt(value)
                break
            case 'hour':
                hour = parseInt(value)
                break
            case 'minute':
                minute = parseInt(value)
                break
        }

        onChanged?.(new Date(year, month - 1, day, hour, minute))
    }
</script>

<div class="flex flex-wrap items-center gap-1">
    <span class="w-14">
        <AutoCompleteInput
            name="year"
            options={async () => {
                return years.map((y) => ({
                    key: y.toString(),
                    value: y.toString(),
                }))
            }}
            value={year.toString()}
            onBlur={(e) => onChange('year', e)}
            onSelect={(opt) => onChange('year', opt.key)}
        />
    </span>

    <span class="text-gray-500">/</span>

    <span class="w-8">
        <AutoCompleteInput
            name="month"
            options={async () => {
                return months.map((m) => ({
                    key: m.toString(),
                    value: m.toString(),
                }))
            }}
            value={month.toString()}
            onBlur={(e) => onChange('month', e)}
            onSelect={(opt) => onChange('month', opt.key)}
        />
    </span>

    <span class="text-gray-500">/</span>

    <span class="w-8">
        <AutoCompleteInput
            name="day"
            options={async () => {
                return dayOptions.map((d) => ({
                    key: d.toString(),
                    value: d.toString(),
                }))
            }}
            value={day.toString()}
            onBlur={(e) => onChange('day', e)}
            onSelect={(opt) => onChange('day', opt.key)}
        />
    </span>

    {#if !dateOnly}
        <span class="text-gray-500">&nbsp;</span>

        <span class="w-8">
            <AutoCompleteInput
                name="hour"
                options={async () => {
                    return hours.map((h) => ({
                        key: h.toString(),
                        value: h.toString(),
                    }))
                }}
                value={hour.toString()}
                onBlur={(e) => onChange('hour', e)}
                onSelect={(opt) => onChange('hour', opt.key)}
            />
        </span>

        <span class="text-gray-500">:</span>

        <span class="w-8">
            <AutoCompleteInput
                name="minute"
                options={async () => {
                    return minutes.map((m) => ({
                        key: m.toString(),
                        value: m.toString(),
                    }))
                }}
                value={minute.toString()}
                onBlur={(e) => onChange('minute', e)}
                onSelect={(opt) => onChange('minute', opt.key)}
            />
        </span>
    {/if}
</div>
