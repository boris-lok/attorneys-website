<script lang="ts">
	import { t } from 'svelte-i18n';

	const googleMapAPIKey = import.meta.env.VITE_GOOGLE_MAP_API_KEY;
	const coordinates = import.meta.env.VITE_COMPANY_CENTER_COORDINATES;
	const placeID = import.meta.env.VITE_COMPANY_PLACE_ID;
	let address = '100506臺北市中正區忠孝西路一段50號20樓之32（亞洲廣場大樓）';
	let phone = '02-23707088';
	let email = 'info@example.com';
</script>

<div class="company-info-section">
	<div class="map-info">
		<iframe
			allowfullscreen
			loading="lazy"
			referrerpolicy="no-referrer-when-downgrade"
			src="https://www.google.com/maps/embed/v1/place?key={googleMapAPIKey}
		&q={placeID}
		&zoom=16
"
			style="border:0"
			title="company map">
		</iframe>
	</div>
	<div class="contact-info">
		<div class="block">
			<h3>{$t('address')}</h3>
			<p>{address}</p>
		</div>
		<div class="block">
			<h3>{$t('phone')}</h3>
			<p><a href="tel:{phone}">{phone}</a></p>
		</div>
		<div class="block">
			<h3>{$t('email')}</h3>
			<p><a href="mailto:{email}">{email}</a></p>
		</div>
	</div>
</div>

<style lang="scss">
  .company-info-section {
    padding: 0 5%;

    iframe {
      width: 100%;
      height: 320px;
      border: none;
    }

    .block {
      margin: 1rem 0;

      h3 {
        font-size: 1.25rem;
        line-height: 1.25rem;
        padding: 0;
      }

      p {
        padding: 0.25rem .5rem;
      }

      a {
        text-decoration: none;
      }
    }
  }

  @media (min-width: 768px) {
    .company-info-section {
      display: flex;
      flex-direction: row-reverse;
      gap: 1rem;
      max-width: 1280px;
      width: 80%;
      margin: 0 auto;
      background-color: white;
      border-radius: 8px;
      padding: 0 0 0 3%;
      overflow: clip;

      iframe {
        width: auto;
        min-width: 480px;
        height: 480px;
      }

      .block:nth-child(1) {
        margin-top: 4rem;
      }

      .block {
        h3 {
          font-size: 1.5rem;
          line-height: 1.5rem;
          padding: 0;
        }

        p {
          padding: 0.25rem .5rem;
          font-size: 1rem;
        }
      }
    }
  }
</style>