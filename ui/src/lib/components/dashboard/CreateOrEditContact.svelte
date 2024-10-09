<script lang="ts">
	import type { Language } from '$lib/models/Language';
	import Input from '$lib/components/Input.svelte';
	import { t } from 'svelte-i18n';

	// If id is not empty, we update the contact resource by id.
	// Otherwise, we create a new contact information
	export let id = '';
	// The address of the contact information
	export let address = '';
	// The phone of the contact information
	export let phone = '';
	// The email of the contact information
	export let email = '';

	let language: Language = 'zh';
	let isLoading = false;

	// An event handler handles the address changed
	function onAddressChanged(e: Event) {
		address = (e.target as HTMLInputElement).value.trim();
	}

	// An event handler handles the phone changed
	function onPhoneChanged(e: Event) {
		phone = (e.target as HTMLInputElement).value.trim();
	}

	// An event handler handles the email changed
	function onEmailChanged(e: Event) {
		email = (e.target as HTMLInputElement).value.trim();
	}
</script>

<div class="contact-form">
	<Input label={$t('address')} name="name" on:input={onAddressChanged} />
	<Input label={$t('phone')} name="name" on:input={onPhoneChanged} />
	<Input label={$t('email')} name="name" on:input={onEmailChanged} />
</div>

<style lang="scss">

</style>