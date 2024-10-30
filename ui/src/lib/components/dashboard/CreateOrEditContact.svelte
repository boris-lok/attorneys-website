<script lang="ts">
	import type { Language } from '$lib/models/Language';
	import Input from '$lib/components/Input.svelte';
	import { t } from 'svelte-i18n';
	import { browser } from '$app/environment';
	import { Contacts } from '$lib/services';
	import { startWithTap } from '$lib/utils';
	import { finalize } from 'rxjs';
	import SpinningLoading from '$lib/components/SpinningLoading.svelte';

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

	// An event handler handles `back` button click
	function onBackButtonClicked() {
		if (browser) {
			window.history.back();
		}
	}

	function validate() {
		// Check if the address is valid
		if (address.trim().length === 0) {
			return false;
		}

		// Check if the phone is valid
		if (phone.trim().length === 0) {
			return false;
		}

		// Check if the email is valid
		return email.match(
			/^(([^<>()[\]\\.,;:\s@\"]+(\.[^<>()[\]\\.,;:\s@\"]+)*)|(\".+\"))@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\])|(([a-zA-Z\-0-9]+\.)+[a-zA-Z]{2,}))$/
		);
	}

	// An event handler handles `create/update` home resource
	function onSubmitButtonClicked() {
		if (isLoading) {
			return;
		}

		if (!validate()) {
			return;
		}

		let json = {
			...id !== '' ? { id: id } : {},
			address: address,
			phone: phone,
			email: email,
			language: language,
			seq: 0
		};

		Contacts.save(json)
			.pipe(
				startWithTap(() => isLoading = true),
				finalize(() => isLoading = false)
			)
			.subscribe();
	}
</script>

<div class="contact-form">

	<div class="full-page-loading-wrapper" class:active={isLoading}>
		<SpinningLoading isLoading={isLoading} />
	</div>

	<Input label={$t('address')} name="name" on:input={onAddressChanged} value={address} />
	<Input label={$t('phone')} name="name" on:input={onPhoneChanged} value={phone} />
	<Input label={$t('email')} name="name" on:input={onEmailChanged} value={email} />
	<div class="btn-container">
		<button class="btn submit" disabled={isLoading} on:click={onSubmitButtonClicked} type="button">{$t('save')}</button>
		<button class="btn back" disabled={isLoading} on:click={onBackButtonClicked} type="button">{$t('back')}</button>
	</div>
</div>

<style lang="scss">
  .contact-form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
    padding: 1rem 5%;
    max-width: 40rem;
    margin: 0 auto;
  }

  .btn-container {
    grid-area: btn-container;
    text-align: center;

    .btn {
      width: 7.5rem;
      height: 2.5rem;
      cursor: pointer;
      margin: 0 0.25rem;

      &.submit {
        border: 1px solid $deep-blue;
      }

      &.back {
        border: 1px solid $deep-red;
      }
    }
  }
</style>