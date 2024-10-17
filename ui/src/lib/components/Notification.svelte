<script lang="ts">
	// The flag that displays the notification message on the UI.
	import type { NotificationType } from '$lib/models/HtmlTag';
	import { notification } from '../../stores/notificationStore';

	let visible = false;
	// The message that we want to display
	let message = 'message';
	// The type of notification (info, error)
	let type: NotificationType = 'info';

	$: {
		$notification; // Automatically subscribes to the store
		({ message, type, visible } = $notification);
	}
</script>

{#if visible}
	<div class="notification {type}">
		<p>{message}</p>
	</div>
{/if}

<style lang="scss">
  .notification {
    display: inline-block;
    position: fixed;
    z-index: $layout-notification-index;
    text-align: center;
    width: 400px;
    left: calc(50% - 200px);
    top: 2rem;
    border-radius: 4px;
    background-color: rgba(66, 66, 66, 0.1);
    backdrop-filter: blur(1rem);

    &.info {
      color: $light-green;
    }

    &.error {
      color: $deep-red;
    }
  }
</style>