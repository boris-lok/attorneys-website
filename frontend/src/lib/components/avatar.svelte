<script lang="ts">
    import type {Avatar} from "../models/member";

    export let avatar: Avatar;
    export let tags: string[] = [];
</script>

<div class="avatar-container">
    <div class="avatar-content">
        <img class="avatar" src="avatar.svg" alt="Avatar"/>
        <p>{ avatar.name }</p>
    </div>
    {#if tags.length > 0}
        <div class="divider"></div>
        <div class="tags-container">
            <div class="tags">
                {#each tags as tag}
                    <p class="tag">{tag}</p>
                {/each}
            </div>
        </div>
    {/if}
</div>

<style lang="scss">
  @import "../../constants";

  div.avatar-container {
    position: relative;
    display: flex;
    flex-direction: column;
    row-gap: 4px;
    padding: 4px 8px;
    margin: 0 16px;
    box-shadow: 0 0 10px 0 rgba(170, 170, 170, .2);
    border-radius: 4px;
    overflow: clip;
    height: 110px;

    p {
      margin: 0;
      padding: 0;
    }

    div.divider {
      width: 100%;
      height: 1px;
      background-color: $grey;
    }

    div.avatar-content {
      position: relative;
      display: flex;
      flex-direction: row;
      column-gap: 6px;
      align-items: center;
      max-width: 100%;
      height: 100%;

      img.avatar {
        width: 64px;
        height: 64px;
        border-radius: 50%;
      }
    }

    div.tags-container {
      display: flex;
      flex-direction: column;
      row-gap: 4px;
      position: relative;
      padding: 4px 8px;

      div.tags {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 4px;

        p.tag {
          font-size: 10px;
          border-radius: 4px;
          border: 1px solid rgba(0, 0, 0, .2);
          padding: 2px 4px;
        }
      }
    }
  }

  @media only screen and (min-width: $breakpoint-sm) {
    div.avatar-container {
      width: 320px;
      max-width: 320px;
    }
  }
</style>