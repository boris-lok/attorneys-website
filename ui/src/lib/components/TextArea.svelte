<script lang="ts">
	export let label = '';
	export let data = '';
</script>

<div class="textarea-wrapper">
	{#if label !== ''}
		<h3>{label}</h3>
	{/if}
	<textarea on:input rows="20">{data}</textarea>
</div>

<style lang="scss">
  .textarea-wrapper {
    position: relative;
    display: flex;
    flex-direction: column;

    textarea {
      width: 100%;
      outline: none;
      resize: none;
      padding: 10px 16px;
      box-sizing: border-box;

      &:focus {
        border-color: $deep-blue;
      }
    }
  }
</style>

