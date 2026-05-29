<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import { UserService } from '$lib/services/user.service'
    import { goto } from '$app/navigation'
    import Loading from '$lib/components/common/Loading.svelte'

    let isLoading = $state(false)
    let username = $state('')
    let password = $state('')

    function validate() {
        if (username === '' || password === '') {
            return false
        }
        return true
    }

    async function onSubmitClicked() {
        if (!validate()) {
            return
        }

        isLoading = true
        const resp = await UserService.login({ username, password })

        if (resp.error) {
            console.error(`login failed: ${resp}`)
            isLoading = false
            return
        }

        await goto(`/b/dashboard`)
    }
</script>

{#if isLoading}
    <Loading />
{:else}
    <div class="mx-auto mt-[10%] mb-4 w-11/12 rounded bg-white px-8 pt-6 pb-8 shadow-md md:w-96">
        <Input
            label="Username"
            name="username"
            type="text"
            bind:value={username}
        />
        <Input
            label="Password"
            name="password"
            type="password"
            bind:value={password}
        />
        <div class="flex items-center justify-center">
            <button
                class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none disabled:bg-gray-500"
                disabled={isLoading}
                onclick={onSubmitClicked}
            >Login
            </button>
        </div>
    </div>
{/if}
