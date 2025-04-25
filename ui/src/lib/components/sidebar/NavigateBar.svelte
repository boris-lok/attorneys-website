<script lang="ts">
    import NavigateItem from '$lib/components/sidebar/NavigateItem.svelte'
    import IconifyIcon from '@iconify/svelte'
    import logo from '$lib/assets/logo.webp'
    import smLogo from '$lib/assets/logo.sm.webp'
    import type { NavigationItem } from '$lib/types'

    type InputProps = {
        rootUrl: string
        items: NavigationItem[]
    }

    let { rootUrl, items }: InputProps = $props()

    // The status of a dropdown menu.
    let show = $state(false)
    let innerWidth = $state(0)


    // handle the menu button clicked.
    function toggleMenu() {
        show = !show
    }

    // remove the classes from the body and set show to false
    function disableDropdownNavigationBar() {
        show = false
        document.body?.classList.remove('overflow-hidden')
        document.body?.classList.remove('h-[calc(100vh-4rem)]')
    }

    // add the classes to body
    function enableDropdownNavigationBar() {
        document.body?.classList.add('overflow-hidden')
        document.body?.classList.add('h-[calc(100vh-4rem)]')
    }

    $effect(() => {
        if (innerWidth > 768) {
            // if the user resizes the window and then
            // the window size is greater than 48rem(768px)
            // we need to hidden the dropdown navigation bar
            // and remove the classes from the body
            disableDropdownNavigationBar()
        }
    })

    $effect(() => {
        if (show) {
            enableDropdownNavigationBar()
        } else {
            disableDropdownNavigationBar()
        }
    })
</script>

<svelte:window bind:innerWidth={innerWidth} />

<nav class="relative z-50">
    <div
        class="relative flex h-16 flex-row items-center gap-12 overflow-hidden bg-[var(--primary-color)] px-4 md:px-8"
    >
        <div
            class="relative flex h-12 w-[100%] flex-row items-center justify-between"
        >
            <!-- Logo -->
            <div>
                <a href={rootUrl}>
                    <picture>
                        <source media="(min-width: 768px)" srcset={logo}>
                        <img alt="logo" class="h-14 md:h-16" src={smLogo} />
                    </picture>

                </a>
            </div>

            <!-- Menu Icon -->
            <div
                class="relative flex items-center justify-center sm:hidden"
            >
                <button class="cursor-pointer" onclick={toggleMenu}>
                    <IconifyIcon
                        class="h-6 w-6 m-2"
                        icon={show
                            ? 'material-symbols-light:close'
                            : 'ri:menu-3-fill'}
                    />
                </button>
            </div>

            <!-- Top Bar Navigate Item -->
            <div
                class="relative flex flex-row max-sm:hidden gap-8"
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

</nav>

<!-- Becuase backdrop-filter: blur causes some issue on mobile, we use background to achieve the same feature -->
<div
    class="absolute h-[calc(100vh-4rem)] translate-y-[calc(-100vh+4rem)] [&.show]:translate-y-0 bg-gray-300/95 w-full transition-[translate,z-index,opacity] duration-500 z-[49] opacity-0 [&.show]:opacity-100 overflow-hidden ease-in-out"
    class:show>

    <!--Dropdown Navigate Item -->
    <div
        class="relative w-screen overflow-y-scroll md:hidden h-[calc(100vh-4rem)]"
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
</div>
