<script lang="ts">
	import { user } from '../../stores/userStore';
	import Input from '$lib/components/Input.svelte';
	import { t } from 'svelte-i18n';
	import { goto } from '$app/navigation';

	// login information
	let username = '';
	let password = '';

	// The flag indicates is calling an API
	let isLoading = false;

	// redirect to dashboard if user is logged in
	$: if ($user) {
		goto('/admin/dashboard');
	}

	// validate username and password are not empty
	function validate() {
		return username.trim() !== '' && password.trim() !== '';
	}

	// handles username has been changed
	function onUsernameChanged(event: Event) {
		username = (event.target as HTMLInputElement).value;
	}

	// handles password has been changed
	function onPasswordChanged(event: Event) {
		password = (event.target as HTMLInputElement).value;
	}

	// handles keyboard press event
	function onKeyPress(event: KeyboardEvent) {
		let key_code = event.code || event.key;
		if (key_code === 'Enter') {
			loginHandler();
		}
	}

	// handles login
	function loginHandler() {
		if (isLoading) {
			return;
		}

		if (!validate()) {
			return;
		}

		user.set(
			JSON.stringify({
				username: username
			})
		);

		goto('/admin/dashboard');
	}

	function onBackButtonClicked() {
		goto('/');
	}

	function onSubmitButtonClicked() {
		loginHandler();
	}

</script>

<div class="login-wrapper">
	<h2>{$t('login')}</h2>
	<Input type="text" name="username" value="" label="Username" on:keypress={onKeyPress} on:input={onUsernameChanged} />
	<Input type="password" name="password" value="" label="Password" on:keypress={onKeyPress}
				 on:input={onPasswordChanged} />

	<div class="btn-container">
		<button class="btn submit" disabled={isLoading} on:click={onSubmitButtonClicked}
						type="button">{$t('login')}</button>
		<button class="btn back" disabled={isLoading} on:click={onBackButtonClicked} type="button">{$t('back')}</button>
	</div>
</div>

<style lang="scss">
  .login-wrapper {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    align-items: center;
    box-shadow: 0 0 8px 0 $deep-grey;
    padding: 1.5rem 0.5rem;
    border-radius: 4px;
    margin: 2rem auto;
  }

  .btn-container {
    grid-area: btn-container;
    text-align: center;
    margin-top: 2rem;

    .btn {
      width: 7.5rem;
      height: 2.5rem;
      cursor: pointer;
      margin: 0 0.25rem;
      background-color: transparent;

      &.submit {
        border: 1px solid $deep-blue;
      }

      &.back {
        border: 1px solid $deep-red;
      }
    }
  }
</style>