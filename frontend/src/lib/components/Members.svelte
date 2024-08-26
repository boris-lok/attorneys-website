<script lang="ts">
    import type {Member} from '../models/member';
    import Avatar from './Avatar.svelte';

    let members: Member[] = [
        {
            id: "1",
            name: "Boris",
            url: "",
            tags: ["Engineer", "Frontend"]
        },
        {
            id: "2",
            name: "Lily",
            url: "",
            tags: []
        }
    ];
</script>

<div class="member-list-container">
    {#each members as member}
        <Avatar avatar={member} tags={member.tags}/>
    {/each}
</div>

<style lang="scss">
  @import "../../constants";

  div.member-list-container {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  @media only screen and (min-width: $breakpoint-md) {
    div.member-list-container {
      flex-direction: row;
    }
  }
</style>