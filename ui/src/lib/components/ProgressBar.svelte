<script lang="ts">
    type Props = {
        progress: number
    }

    let { progress }: Props = $props()
    const barColor = $derived.by(() => {
        return progress > 90
            ? 'bg-red-500'
            : progress > 70
              ? 'bg-orange-500'
              : 'bg-green-500'
    })
</script>

<div class="h-2 w-full rounded bg-gray-200">
    <div class={`${barColor} h-2 rounded`} style:width={`${progress}%`}></div>
</div>
