<script lang="ts">
    import type {Avatar} from "../models/member";

    export let avatar: Avatar;
    export let tags: string[] = ["Rust", "Python", "C++", "Go", "Typescript", "Scala", "Kotlin"];
</script>

<main>
    <div class="avatar-container">
        <img class="avatar" src="avatar.svg" alt="Avatar"/>
        <p>{ avatar.name }</p>
    </div>
    {#if tags.length > 0}
        <div class="divider"></div>
        <div class="content">
            <div class="tags">
                {#each tags as tag}
                    <p class="tag">{tag}</p>
                {/each}
            </div>
        </div>
    {/if}
</main>

<style lang="scss">
  main {
    position: relative;
    display: flex;
    flex-direction: column;
    row-gap: 4px;
    padding: 4px 8px;
    margin: 0 16px;
    box-shadow: 0 0 16px 0 rgba(0, 0, 0, .15);
    border-radius: 4px;
    overflow: clip;

    p {
      margin: 0;
      padding: 0;
    }

    div.divider {
      width: 100%;
      height: 1px;
      background-color: rgba(0,0,0,.2);
    }

    div.avatar-container {
      position: relative;
      display: flex;
      flex-direction: row;
      column-gap: 6px;
      align-items: center;
      max-width: 100%;


      img.avatar {
        width: 64px;
        height: 64px;
        border-radius: 50%;
      }
    }

    div.content {
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
</style>