<script lang="ts">
    interface Parts {
        year: string;
        month: string;
        day: string;
        hour: string;
        minute: string;
    }

    interface Props {
        value?: Date | null;
        showTime?: boolean;
        onchange?: (value: Date | null) => void;
    }

    let {
        value = $bindable(null),
        showTime = false,
        onchange
    }: Props = $props();

    function partsFromDate(d: Date | null): Parts {
        if (!(d instanceof Date) || isNaN(d.getTime())) {
            return { year: '', month: '', day: '', hour: '', minute: '' };
        }
        return {
            year: String(d.getFullYear()),
            month: String(d.getMonth() + 1).padStart(2, '0'),
            day: String(d.getDate()).padStart(2, '0'),
            hour: String(d.getHours()).padStart(2, '0'),
            minute: String(d.getMinutes()).padStart(2, '0')
        };
    }

    let parts = $state<Parts>(partsFromDate(value));

    // Track the date this component last produced, so we can tell an
    // external `value` change apart from the user's own typing.
    let lastEmitted: Date | null = value;

    $effect(() => {
        // Re-sync the visible fields only when the parent changed 'value'.
        if (value?.getTime() !== lastEmitted?.getTime()) {
            lastEmitted = value;
            parts = partsFromDate(value);
        }
    });

    let yearEl: HTMLInputElement | undefined = $state();
    let monthEl: HTMLInputElement | undefined = $state();
    let dayEl: HTMLInputElement | undefined = $state();
    let hourEl: HTMLInputElement | undefined = $state();
    let minuteEl: HTMLInputElement | undefined = $state();

    function clampDigits(str: string, maxLen: number): string {
        return str.replace(/\D/g, '').slice(0, maxLen);
    }

    function buildDate(): Date | null {
        const { year, month, day, hour, minute } = parts;
        if (!year || !month || !day) return null;
        const y = +year;
        const mo = +month - 1;
        const d = +day;
        const h = showTime ? +(hour || 0) : 0;
        const mi = showTime ? +(minute || 0) : 0;
        const dt = new Date(y, mo, d, h, mi);
        if (isNaN(dt.getTime())) return null;
        // Reject overflow (e.g. month 13, day 32, Feb 30)
        if (dt.getMonth() !== mo || dt.getDate() !== d) return null;
        return dt;
    }

    function commit() {
        value = buildDate();
        lastEmitted = value;
        onchange?.(value);
    }

    function handle(field: keyof Parts, maxLen: number, nextEl?: HTMLInputElement) {
        return (e: Event & { currentTarget: HTMLInputElement }) => {
            const cleaned = clampDigits(e.currentTarget.value, maxLen);
            parts[field] = cleaned;
            e.currentTarget.value = cleaned;
            commit();
            if (cleaned.length === maxLen && nextEl) {
                nextEl.focus();
                nextEl.select();
            }
        };
    }

    function handleBackspace(prevEl?: HTMLInputElement) {
        return (e: KeyboardEvent & { currentTarget: HTMLInputElement }) => {
            if (e.key === 'Backspace' && e.currentTarget.value === '' && prevEl) {
                prevEl.focus();
            }
        };
    }

    function padOnBlur(field: keyof Parts) {
        return () => {
            if (parts[field].length === 1) {
                parts[field] = parts[field].padStart(2, '0');
                commit();
            }
        };
    }
</script>

<div
    class="inline-flex items-center gap-0.5 border-b px-2 py-1 font-mono text-gray-700 focus-within:border-b-blue-500"
>
    <input
        bind:this={yearEl}
        value={parts.year}
        oninput={handle('year', 4, monthEl)}
        placeholder="YYYY"
        inputmode="numeric"
        size="4"
        class="w-12 appearance-none bg-transparent text-center placeholder:text-gray-400 focus:outline-none"
    />
    <span class="text-gray-400">/</span>
    <input
        bind:this={monthEl}
        value={parts.month}
        oninput={handle('month', 2, dayEl)}
        onkeydown={handleBackspace(yearEl)}
        onblur={padOnBlur('month')}
        placeholder="MM"
        inputmode="numeric"
        size="2"
        class="w-7 appearance-none bg-transparent text-center placeholder:text-gray-400 focus:outline-none"
    />
    <span class="text-gray-400">/</span>
    <input
        bind:this={dayEl}
        value={parts.day}
        oninput={handle('day', 2, showTime ? hourEl : undefined)}
        onkeydown={handleBackspace(monthEl)}
        onblur={padOnBlur('day')}
        placeholder="DD"
        inputmode="numeric"
        size="2"
        class="w-7 appearance-none bg-transparent text-center placeholder:text-gray-400 focus:outline-none"
    />

    {#if showTime}
        <span class="w-2"></span>
        <input
            bind:this={hourEl}
            value={parts.hour}
            oninput={handle('hour', 2, minuteEl)}
            onkeydown={handleBackspace(dayEl)}
            onblur={padOnBlur('hour')}
            placeholder="HH"
            inputmode="numeric"
            size="2"
            class="w-7 appearance-none bg-transparent text-center placeholder:text-gray-400 focus:outline-none"
        />
        <span class="text-gray-400">:</span>
        <input
            bind:this={minuteEl}
            value={parts.minute}
            oninput={handle('minute', 2, undefined)}
            onkeydown={handleBackspace(hourEl)}
            onblur={padOnBlur('minute')}
            placeholder="mm"
            inputmode="numeric"
            size="2"
            class="w-7 appearance-none bg-transparent text-center placeholder:text-gray-400 focus:outline-none"
        />
    {/if}
</div>
