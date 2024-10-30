<script lang="ts">

	export let value = 0;
	export let has_error = false;
	export let name = '';
	export let label = '';
	export let placeholder = '';

	export const reset = () => (ref.value = '');

	let ref: HTMLInputElement;
</script>

<div class="input-box-container">
	<input
		bind:this={ref}
		class:error={has_error}
		{name}
		on:blur
		on:input
		on:keypress
		placeholder=" "
		type="number"
		{value}
	/>
	<span>{label}</span>
	{#if placeholder !== ""}
		<p>{placeholder}</p>
	{/if}
</div>

<style lang="scss">
  .input-box-container {
    position: relative;
    width: 100%;

    > input {
      width: 100%;
      padding: 10px 16px;
      border-radius: 4px;
      outline: none;
      font-size: 1em;
      border: 1px solid $black;
      box-sizing: border-box;

      &:focus ~ span,
      &:not(:placeholder-shown) ~ span {
        color: $deep-blue;
        transform: translateX(8px) translateY(-12px);
        font-size: 0.75em;
        padding: 0 16px;
        background-color: white;
        border-radius: 4px;
      }

      &:focus {
        border: 1px solid $deep-blue;
      }

      & .error {
        border: 1px solid $deep-red;
      }
    }

    > span {
      position: absolute;
      left: 0;
      top: 0;
      padding: 10px;
      pointer-events: none;
      font-size: 1em;
      transition: all 0.3s;
    }

    > p {
      font-size: 0.9rem;
      line-height: 1rem;
      padding: 0.5rem;
      color: $deep-orange;
    }
  }

  // disable input[type=number] step
  input[type=number]::-webkit-inner-spin-button,
  input[type=number]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  input[type=number] {
    -moz-appearance: textfield; // same as -webkit-appearance: none in Firefox
  }
</style>