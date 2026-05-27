<script lang="ts">

    import Input from '$lib/components/common/Input.svelte'

    type Props = {
        date?: Date
        onChanged?: (date: Date) => void
        dateOnly?: boolean
    }
    let { date = new Date(), onChanged, dateOnly }: Props = $props()

    let year = $derived(date.getFullYear())
    let month = $derived(date.getMonth() + 1)
    let day = $derived(date.getDate())
    let hour = $derived(date.getHours())
    let minute = $derived(date.getMinutes())

    function onChange(
        key: 'year' | 'month' | 'day' | 'hour' | 'minute',
        value: string
    ) {
        const parsedValue = parseInt(value)

        if (isNaN(parsedValue)) return


        switch (key) {
            case 'year':
                onChanged?.(new Date(parsedValue, month - 1, day, hour, minute))
                break
            case 'month':
                onChanged?.(new Date(year, parsedValue - 1, day, hour, minute))
                break
            case 'day':
                onChanged?.(new Date(year, month - 1, parsedValue, hour, minute))
                break
            case 'hour':
                onChanged?.(new Date(year, month - 1, day, parsedValue, minute))
                break
            case 'minute':
                onChanged?.(new Date(year, month - 1, day, hour, parsedValue))
                break
        }

    }

</script>

<div class="flex flex-wrap items-center gap-1">
    <span class="w-14">
        <Input
            name="year"
            value={year.toString()}
            type="text"
            onInput={(e) => onChange('year', e.currentTarget.value)}
        />
    </span>

    <span class="text-gray-500">/</span>

    <span class="w-8">
               <Input
                   name="year"
                   value={year.toString()}
                   type="text"
                   onInput={(e) => onChange('month', e.currentTarget.value)}
               />
    </span>

    <span class="text-gray-500">/</span>

    <span class="w-8">
               <Input
                   name="year"
                   value={year.toString()}
                   type="text"
                   onInput={(e) => onChange('day', e.currentTarget.value)}
               />
    </span>

    {#if !dateOnly}
        <span class="text-gray-500">&nbsp;</span>

        <span class="w-8">
                          <Input
                              name="year"
                              value={year.toString()}
                              type="text"
                              onInput={(e) => onChange('hour', e.currentTarget.value)}
                          />
        </span>

        <span class="text-gray-500">:</span>

        <span class="w-8">
                          <Input
                              name="year"
                              value={year.toString()}
                              type="text"
                              onInput={(e) => onChange('minute', e.currentTarget.value)}
                          />
        </span>
    {/if}
</div>
