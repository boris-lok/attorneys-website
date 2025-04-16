<script lang="ts">
    import IconifyIcon from '@iconify/svelte'

    const googleMapAPIKey = import.meta.env.VITE_GOOGLE_MAP_API_KEY
    const placeID = import.meta.env.VITE_COMPANY_PLACE_ID

    // The address wants to show in the UI
    let address = '台北市中山區中山北路二段162號10樓'
    // The email address wants to show in the UI
    let email = 'info@chenwanglaw.com'
    let line = '@594ylsoj'
</script>

<div
    class="relative flex h-[calc(100vh-144px-96px)] min-h-[48rem] flex-col items-center"
>
    <p class="px-16 pt-16 text-4xl font-bold text-[var(--primary-color)]">
        聯絡我們
    </p>
    <div
        class="relative flex flex-col md:mx-auto md:mt-12 md:w-full md:max-w-4xl md:flex-col md:gap-8"
    >

        <div class="flex flex-col gap-4 px-4 py-4 md:flex-1 md:justify-evenly items-center">
            <div class="flex flex-row items-center gap-2">
                <IconifyIcon
                    class="h-12 w-12"
                    icon="mdi:address-marker-outline"
                />
                <p class="text-2xl">{address}</p>
            </div>
            <div class="flex flex-row items-center gap-2">
                <IconifyIcon class="h-12 w-12" icon="formkit:email" />
                <a class="text-2xl" href="mailto:{email}">{email}</a>
            </div>
        </div>

        <div class="w-full overflow-clip rounded-xl min-h-48 md:min-h-96 lg:min-h-96 md:flex-1 m-auto md:max-w-96 lg:max-w-[768px]">
            <iframe
                allowfullscreen
                loading="lazy"
                referrerpolicy="no-referrer-when-downgrade"
                src="https://www.google.com/maps/embed/v1/place?key={googleMapAPIKey}
		&q={placeID}
		&zoom=16
"
                style="border:0;width:100%;height: 100%;"
                title="company map"
            >
            </iframe>
        </div>
    </div>
</div>
