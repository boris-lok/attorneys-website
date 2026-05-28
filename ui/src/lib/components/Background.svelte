<script lang="ts">
    import bgImage from '$lib/assets/bg-image.webp'
    import smBgImage from '$lib/assets/bg-image.sm.webp'
    import IconifyIcon from '@iconify/svelte'

    type ContactUsType = {
        icon: string
        link: string
    }

    let contactUs: ContactUsType[] = [
        {
            icon: 'cib:line',
            link: 'https://lin.ee/NW5iyIk',
        },
        {
            icon: 'skill-icons:instagram',
            link: 'https://www.instagram.com/chenwanglaw?igsh=MW0yY3pkbzlxN2lidQ%3D%3D&utm_source=qr',
        },
        {
            icon: 'devicon:facebook',
            link: 'https://www.facebook.com/share/1BfUcyRZki/?mibextid=wwXIfr&ref=xav_pl_fb_external_link_ios',
        },
    ]
</script>

<div class="relative flex h-36 justify-center overflow-clip md:h-[480px]">
    <picture>
        <source media="(min-width: 768px)" srcset={bgImage} />
        <img
            alt="bg"
            class="absolute inset-0 z-0 h-36 w-full animate-[--brightness_3s_ease-in-out] object-cover brightness-75 md:h-[480px]"
            src={smBgImage}
        />
    </picture>

    <div class="z-10 pt-4 text-center md:pt-36">
        <p class="text-2xl text-gray-300 md:text-4xl md:text-6xl lg:text-6xl">勤睿國際法律事務所</p>

        <div
            class="fixed right-[1.5rem] bottom-[3rem] flex flex-col md:mt-8 lg:relative lg:right-auto lg:bottom-auto"
        >
            <div class="flex flex-col gap-6 lg:flex-row lg:gap-16">
                {#each contactUs as item (item.link)}
                    <a
                        class="flex w-fit flex-row items-center gap-4 rounded-[50%] border-yellow-100 bg-white p-2 shadow-xl lg:rounded lg:border lg:bg-transparent lg:px-8 lg:py-4 lg:shadow-none"
                        href={item.link}
                        target="_blank"
                    >
                        <IconifyIcon
                            class="h-8 w-8 text-green-400 lg:h-10 lg:w-10"
                            icon={item.icon}
                        />
                        <p class="hidden lg:block lg:text-3xl lg:text-xl lg:text-yellow-100">
                            諮詢
                        </p>
                    </a>
                {/each}
            </div>
        </div>
    </div>
</div>
