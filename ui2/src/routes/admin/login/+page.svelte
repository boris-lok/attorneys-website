<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'
    import { UserService } from '$lib/services/user.service'

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

    function onClick() {
        if (data.username === '' && data.password === '') {
            return
        }

        UserService.login(data).subscribe({
            next: (resp) => {
                if (resp.error === true) {
                    // TODO: show error message.
                    console.error(resp)

                    return
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
            class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none"
            onclick={onClick}
            >Login
        </button>
    </div>
</div>
