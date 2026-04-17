<script lang="ts">

    import type { PageProps } from './$types'
    import DateTimePicker from '$lib/components/DateTimePicker.svelte'

    let { data }: PageProps = $props()
    let id = data.id
</script>


<DateTimePicker />