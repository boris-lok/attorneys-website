<script lang="ts">
    import bgImage from '$lib/assets/bg-image.png'
</script>

<div class="relative flex h-36 justify-center overflow-clip md:h-[480px]">
    <img
        alt="bg"
        class="brightness absolute z-0 h-36 w-full object-cover brightness-75 md:h-[480px]"
        src={bgImage}
    />

    <div class="z-10 pt-4 md:pt-24">
        <p class="text-2xl text-gray-300 md:text-6xl">勤睿國際法律事務所</p>
    </div>
</div>

<style lang="postcss">
    @keyframes brightness {
        from {
            filter: brightness(0);
        }
        to {
            filter: brightness(0.75);
        }
    }

    .brightness {
        animation: brightness 3s ease-in-out;
    }
</style>