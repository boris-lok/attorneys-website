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

<div class="relative flex flex-col items-center h-[calc(100vh-144px-96px)] min-h-[48rem]">
    <p class="text-4xl px-16 pt-16 text-[var(--primary-color)] font-bold">聯絡我們</p>
    <div class="relative flex flex-col md:w-full md:flex-row-reverse md:gap-8 md:mt-12 md:max-w-4xl md:mx-auto">
        <div class="w-full md:h-96 rounded-xl overflow-clip md:flex-1">
            <iframe
                allowfullscreen
                loading="lazy"
                referrerpolicy="no-referrer-when-downgrade"
                src="https://www.google.com/maps/embed/v1/place?key={googleMapAPIKey}
		&q={placeID}
		&zoom=16
"
                style="border:0;width:100%;height: 100%;"
                title="company map">
            </iframe>
        </div>
        <div class="flex flex-col px-4 py-4 gap-4 md:flex-1 md:justify-evenly">
            <div class="flex flex-row gap-2 items-center">
                <IconifyIcon class="h-8 w-8" icon="mdi:address-marker-outline" />
                <p>{address}</p>
            </div>
            <div class="flex flex-row gap-2 items-center">
                <IconifyIcon class="h-8 w-8" icon="fontisto:line" />
                <a href="https://lin.ee/NW5iyIk">{line}</a>
            </div>
            <div class="flex flex-row gap-2 items-center">
                <IconifyIcon class="h-8 w-8" icon="formkit:email" />
                <a href="mailto:{email}">{email}</a>
            </div>
        </div>
    </div>
</div>
