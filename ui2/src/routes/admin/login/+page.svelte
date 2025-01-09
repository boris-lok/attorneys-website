<script lang="ts">
    import Input from '$lib/components/common/Input.svelte'

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
            [key]: value,
        }
    }
</script>

<div class="mb-4 rounded bg-white px-8 pb-8 pt-6 shadow-md">
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
</div>
