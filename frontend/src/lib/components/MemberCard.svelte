<script lang="ts">
    import type {CardType} from "../models/member";

    export let title: string = "";
    export let data: string[] | string;
    export let type: CardType = "list";
</script>

<div class="card-container">
    {#if title !== ""}
        <div class="title"><p>{ title }</p></div>
    {/if}
    {#if type === "list" }
        <div class="list-content">
            <ul>
                {#each data as item}
                    <li>{ item }</li>
                {/each}
            </ul>
        </div>
    {:else if type === "string" }
        <div class="string-content">
            <p>{data}</p>
        </div>
    {/if}
</div>

<style lang="scss">
  div.card-container {
    display: flex;
    flex-direction: column;
    position: relative;
    width: 100%;
    padding: 16px;
    box-shadow: 0 0 10px 3px rgba(170, 170, 170, .2);
    box-sizing: border-box;

    div.title {
      p {
        padding: 0;
        margin: 0;
        font-size: 22px;
        font-weight: 900;
      }
    }

    div.list-content {
      ul {
        list-style-type: none;
        padding: 0;
        margin: 0;

        li {
          font-size: 16px;
          font-weight: 400;
        }
      }
    }
  }
</style>