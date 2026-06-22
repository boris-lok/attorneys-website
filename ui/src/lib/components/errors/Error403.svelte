<script lang="ts">
    type InputProps = {
        message?: string
    }
    let { message }: InputProps = $props()
</script>

<main>
    <h2>Permission Denied</h2>
    <p>You don't have the permission to access the page.</p>
    <br />
    <p>Error message: {message ?? ''}</p>
</main>