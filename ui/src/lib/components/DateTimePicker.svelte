<script lang="ts">

    type Props = {
        onChanged?: (date: Date) => void
    }
    let { onChanged }: Props = $props()

    let now = new Date()

    let year = $state(now.getFullYear())
    let month = $state(now.getMonth() + 1)
    let day = $state(now.getDate())
    let hour = $state(now.getHours())
    let minute = $state(now.getMinutes())

    const years = Array.from({ length: 10 }, (_, i) => year + i)
    const months = Array.from({ length: 12 }, (_, i) => i + 1)
    const hours = Array.from({ length: 24 }, (_, i) => i)
    const minutes = Array.from({ length: 60 }, (_, i) => i)

    const days = $derived(new Date(year, month, 0).getDate())
    const dayOptions = $derived(Array.from({ length: days }, (_, i) => i + 1))

    $effect(() => {
        if (onChanged) {
            onChanged(new Date(year, month - 1, day, hour, minute))
        }
    })
</script>

<div class="flex gap-2 flex-wrap">
    <select bind:value={year}
            class="appearance-none w-12 border border-gray-300 rounded px-2 curosor-pointer"
    >
        {#each years as y}
            <option value={y}>{y}</option>
        {/each}
    </select>

    <span class="text-gray-500">/</span>

    <select bind:value={month}
            class="appearance-none w-8 border border-gray-300 rounded px-2"
    >
        {#each months as m}
            <option value={m}>{m}</option>
        {/each}
    </select>

    <span class="text-gray-500">/</span>

    <select bind:value={day}
            class="appearance-none w-8 border border-gray-300 rounded px-2"
    >
        {#each dayOptions as d}
            <option value={d}>{d}</option>
        {/each}
    </select>

    <span class="text-gray-500">&nbsp;</span>

    <select bind:value={hour}
            class="appearance-none w-8 border border-gray-300 rounded px-2"
    >
        {#each hours as h}
            <option value={h}>{h.toString().padStart(2, '0')}</option>
        {/each}
    </select>

    <span class="text-gray-500">:</span>

    <select bind:value={minute}
            class="appearance-none w-8 border border-gray-300 rounded px-2"
    >
        {#each minutes as m}
            <option value={m}>{m.toString().padStart(2, '0')}</option>
        {/each}
    </select>
</div>
