<script lang="ts">
    import type {ButtonClass} from "$lib/models/HtmlTag";

    export let text = "";
    export let loading = false;
    export let classname: ButtonClass = "primary-blue";
</script>

{#if loading}
{:else}
    <button type="button" on:click class="{classname}">{text}</button>
{/if}

<style lang="scss">
  button {
    width: 120px;
    height: 36px;
    font-size: 16px;
    cursor: pointer;
    border-radius: 4px;
    border: none;

    &.primary-blue {
      box-shadow: 0 0 4px 0 $light-blue;
      background-color: $deep-blue;
      color: $white;
    }

    &.primary-orange {
      box-shadow: 0 0 4px 0 $light-orange;
      background-color: $deep-orange;
      color: $white;
    }

    &.primary-red {
      box-shadow: 0 0 4px 0 $light-red;
      background-color: $deep-red;
      color: $white;
    }
  }
</style>