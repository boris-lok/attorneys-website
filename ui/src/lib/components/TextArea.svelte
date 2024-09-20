<script lang="ts">
	export let label = '';
</script>

<div class="textarea-wrapper">
	<label>{label}</label>
	<textarea on:input rows="20"></textarea>
</div>

<style lang="scss">
  .textarea-wrapper {
    position: relative;
    display: flex;
    flex-direction: column;

    label {
      pointer-events: none;
      font-size: 1em;
      padding: 0 0 4px 4px;
    }

    textarea {
      width: 100%;
      outline: none;
      resize: none;
      border: 1px solid $black;
      border-radius: 4px;
      padding: 10px 16px;
      box-sizing: border-box;

      &:focus {
        border-color: $deep-blue;
      }
    }
  }
</style>

