<script lang="ts">
	import { t } from 'svelte-i18n';

	let address = '100506臺北市中正區忠孝西路一段50號20樓之32（亞洲廣場大樓）';
	let phone = '02-23707088';
</script>

<footer>
	<div class="contact-us-wrapper">
		<h3>{$t('contact_us')}</h3>
		<span><a href="tel:{phone}">{phone}</a></span>
		<span class="content">{address}</span>
	</div>
</footer>

<style lang="scss">
  footer {
    width: 100%;
    display: flex;
    border: 1px solid rgba(25, 27, 38, .08);
    background-color: rgba(25, 27, 38, .02);

    .contact-us-wrapper {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
      align-items: center;
      padding: 1.25rem 5%;
      width: 100%;

      a, span {
        text-decoration: none;
        color: $black;
        font-size: 1rem;
      }
    }
  }
</style>