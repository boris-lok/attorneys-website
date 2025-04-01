<script lang="ts">
    import bgImage from '$lib/assets/bg-image.webp'
    import smBgImage from '$lib/assets/bg-image.sm.webp'
</script>

<div class="relative flex h-36 justify-center overflow-clip md:h-[480px]">
    <picture>
        <source media="(min-width: 768px)" srcset={bgImage}>
        <img
            alt="bg"
            class="absolute inset-0 z-0 h-36 w-full animate-[--brightness_3s_ease-in-out] object-cover brightness-75 md:h-[480px]"
            src={smBgImage}
        />
    </picture>

    <div class="z-10 pt-4 md:pt-24">
        <p class="text-2xl text-gray-300 md:text-6xl">勤睿國際法律事務所</p>
    </div>
</div>
