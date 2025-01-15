<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import { UserService } from '$lib/services/user.service'
    import { startWithTap } from '$lib/utils'
    import { finalize } from 'rxjs'
    import { goto } from '$app/navigation'
    import { user } from '$lib/stores/user.store'

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

    function onSubmitClicked() {
        if (data.username === '' || data.password === '') {
            return
        }

        UserService.login(data)
            .pipe(
                startWithTap(() => (formLoading = true)),
                finalize(() => (formLoading = false)),
            )
            .subscribe({
                next: (resp) => {
                    console.log(`response: ${JSON.stringify(resp)}`)

                    if (resp.error) {
                        console.error(`login failed: ${JSON.stringify(resp)}`)
                    } else if (!resp.error && 'data' in resp) {
                        user.set(resp.data)
                        goto('/admin/dashboard')
                    }
                },
            })
    }
</script>

<div
    class="mx-auto mb-4 mt-[10%] w-11/12 rounded bg-white px-8 pb-8 pt-6 shadow-md md:w-96"
>
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
            onclick={onSubmitClicked}
            >Login
        </button>
    </div>
</div>
