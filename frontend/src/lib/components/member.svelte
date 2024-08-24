<script lang="ts">
    import type {Avatar as avatar} from '../models/member';
    import Avatar from './avatar.svelte';

    let members: avatar[] = [
        {
            id: "1",
            name: "Boris", url: ""
        }
    ];
</script>

<main>
    <p>Member List page</p>
    {#each members as member}
        <Avatar avatar={member}/>
    {/each}
</main>