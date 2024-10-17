<script lang="ts">
	import type { InputBoxType } from '$lib/models/HtmlTag';

	export let value = '';
	export let has_error = false;
	export let name = '';
	export let type: InputBoxType = 'text';
	export let label = '';

	export const reset = () => (ref.value = '');

	let ref: HTMLInputElement;

	const type_action = (node: HTMLInputElement) => {
		node.type = type;
	};
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
		type="text"
		use:type_action
		{value}
	/>
	<span>{label}</span>
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
  }
</style>