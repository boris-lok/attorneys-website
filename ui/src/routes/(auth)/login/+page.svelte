<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import { UserService } from '$lib/services/user.service'
    import { goto } from '$app/navigation'
    import LoadingButton from '$lib/components/common/LoadingButton.svelte'

    let isLoading = $state(false)
    let username = $state('')
    let password = $state('')
    let errorMsg = $state('')

    function validate() {
        if (username.trim() === '') {
            errorMsg = 'Username is required'
            return false
        }

        if (password === '') {
            errorMsg = 'Password is required'
            return false
        }

        return true
    }

    async function onSubmitClicked() {
        // reset the error message
        errorMsg = ''

        // validate the input fields
        if (!validate()) {
            return
        }

        // Start an API call
        isLoading = true
        try {
            const resp = await UserService.login({ username, password })

            if (resp.error) {
                errorMsg = 'Invalid credentials'
                return
            }

            await goto(`/b/dashboard`)
        } finally {
            isLoading = false
        }
    }
</script>

<div class="mx-auto mt-[10%] mb-4 w-11/12 rounded bg-white px-8 pt-6 pb-8 shadow-md md:w-96">
    {#if errorMsg}
        <p class="mt-2 mb-4 text-center text-red-500">{errorMsg}</p>
    {/if}

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
        <LoadingButton
            isLoading={isLoading}
            onclick={onSubmitClicked}
            loadingText="Logging in..."
        >
            <p>Login</p>
        </LoadingButton>

    </div>
</div>