<script lang="ts">
    import NavigateItem from '$lib/components/sidebar/NavigateItem.svelte'
    import IconifyIcon from '@iconify/svelte'
    import type { NavigationItem } from '$lib/types'
    import logo from '$lib/assets/logo.png'

    let { items }: { items: NavigationItem[] } = $props()

    // The status of dropdown menu.
    let show = $state(false)

    // handle the menu button clicked.
    function toggleMenu() {
        show = !show
    }
</script>

<nav class="relative">
    <div
        class="relative flex h-16 flex-row items-center gap-12 overflow-hidden bg-[var(--primary-color)] px-4 md:px-8"
    >
        <div
            class="relative flex h-12 w-[100%] flex-row items-center justify-between"
        >
            <!-- Logo -->
            <div>
                <img class="h-14 md:h-16" alt="logo" src={logo} />
            </div>

            <!-- Menu Icon -->
            <div
                class="relative flex h-6 w-6 items-center justify-center sm:hidden"
            >
                <button class="cursor-pointer" onclick={toggleMenu}>
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
                    {#if 'onClick' in item}
                        <button
                            class="outline-none"
                            onclick={() => {
                                show = false
                                item.onClick()
                            }}
                        >
                            <NavigateItem
                                label={item.name}
                                icon={item.icon}
                                topBar={true}
                            />
                        </button>
                    {:else if 'url' in item}
                        <a href={item.url} onclick={() => (show = false)}>
                            <NavigateItem
                                label={item.name}
                                icon={item.icon}
                                topBar={true}
                            />
                        </a>
                    {/if}
                {/each}
            </div>
        </div>
    </div>
    <!-- Dropdown Navigate Item -->
    <div
        class="absolute top-16 z-50 h-0 w-screen overflow-y-scroll opacity-0 backdrop-blur-sm transition-[height,opacity] duration-300 md:hidden [&.show]:h-[calc(100vh-4rem)] [&.show]:opacity-100"
        class:show
    >
        <div
            class="grid max-h-[calc(100vh-4rem)] w-full grid-cols-2 justify-items-center gap-y-8 overflow-x-hidden overflow-y-auto px-4 py-6"
        >
            {#each items as item (item.name)}
                {#if 'onClick' in item}
                    <button
                        class="outline-none"
                        onclick={() => {
                            show = false
                            item.onClick()
                        }}
                    >
                        <NavigateItem
                            label={item.name}
                            icon={item.icon}
                            topBar={false}
                        />
                    </button>
                {:else if 'url' in item}
                    <a href={item.url} onclick={() => (show = false)}>
                        <NavigateItem
                            label={item.name}
                            icon={item.icon}
                            topBar={false}
                        />
                    </a>
                {/if}
            {/each}
        </div>
    </div>
</nav>
