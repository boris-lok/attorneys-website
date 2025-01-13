<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import type { ActionData } from './$types'
    import { enhance, applyAction } from '$app/forms'

    let { form }: { form: ActionData } = $props()
    let formLoading = $state(false)

    type Data = {
        username: string
        password: string
    }

    let data: Data = {
        username: '',
        password: '',
    }

    function onDataChanged<K extends keyof Data>(
        key: K,
        e: Event & { currentTarget: EventTarget & HTMLInputElement },
    ) {
        if (!e.target) {
            return
        }

        const { value } = e.target as HTMLInputElement
        data = {
            ...data,
            [key]: value.trim(),
        }
    }
</script>

<div
    class="mx-auto mb-4 mt-[10%] w-11/12 rounded bg-white px-8 pb-8 pt-6 shadow-md md:w-96"
>
    <form
        method="POST"
        use:enhance={() => {
            formLoading = true
            return async ({ result }) => {
                formLoading = false
                await applyAction(result)
            }
        }}
    >
        {#if form?.incorrect}
            <p class="text-center text-xs italic text-red-500">
                Invalid Credentials
            </p>
        {/if}
        {#if formLoading}
            <p class="text-center text-xs italic text-green-500">Loading...</p>
        {/if}
        <Input
            hasError={false}
            label="Username"
            name="username"
            onInput={(e) => onDataChanged('username', e)}
            type="text"
            value=""
        />
        <Input
            hasError={false}
            label="Password"
            name="password"
            onInput={(e) => onDataChanged('password', e)}
            type="password"
            value=""
        />
        <div class="flex items-center justify-center">
            <button
                class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:bg-gray-500"
                disabled={formLoading}
                >Login
            </button>
        </div>
    </form>
</div>
