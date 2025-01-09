<script lang="ts">
    import NavigateItem from '$lib/components/sidebar/NavigateItem.svelte'
    import IconifyIcon from '@iconify/svelte'

    let items = [
        {
            icon: 'iconoir:home',
            name: 'Home',
        },
        {
            icon: 'lsicon:list-outline',
            name: 'Services',
        },
        {
            icon: 'lineicons:books-2',
            name: 'Articles',
        },
        {
            icon: 'tdesign:member',
            name: 'Members',
        },
        {
            icon: 'hugeicons:contact-02',
            name: 'Contact us',
        },
    ]

    // The status of dropdown menu.
    let show = $state(false)

    // handle the menu button clicked.
    function toggleMenu() {
        show = !show
    }
</script>

<nav class="relative">
    <div
        class="relative flex h-16 flex-row items-center gap-12 overflow-hidden bg-gray-400 px-8"
    >
        <div
            class="relative flex h-12 w-[100%] flex-row items-center justify-between"
        >
            <!-- Logo -->
            <div>
                <p class="text-lg">Logo</p>
            </div>

            <!-- Menu Icon -->
            <div
                class="relative flex h-6 w-6 items-center justify-center sm:hidden"
            >
                <button onclick={toggleMenu}>
                    <IconifyIcon
                        class="h-6 w-6"
                        icon={show
                            ? 'material-symbols-light:close'
                            : 'ri:menu-3-fill'}
                    />
                </button>
            </div>

            <!-- Top Bar Navigate Item -->
            <div
                class="relative flex flex-row max-sm:hidden sm:gap-4 md:gap-8 xl:gap-12"
            >
                {#each items as item (item.name)}
                    <NavigateItem
                        label={item.name}
                        icon={item.icon}
                        topBar={true}
                    />
                {/each}
            </div>
        </div>
    </div>
    <!-- Dropdown Navigate Item -->
    <div
        class="absolute top-16 h-0 w-screen overflow-y-scroll opacity-0 backdrop-blur-sm transition-[height,opacity] duration-300 md:hidden"
        class:show
    >
        <div
            class="grid h-[32rem] w-full grid-cols-2 items-center justify-items-center pt-6"
        >
            {#each items as item (item.name)}
                <NavigateItem
                    label={item.name}
                    icon={item.icon}
                    topBar={false}
                />
            {/each}
        </div>
    </div>
</nav>

<style lang="postcss">
    nav {
        .show {
            height: calc(100vh - 4rem);
            opacity: 1;
        }
    }
</style>
