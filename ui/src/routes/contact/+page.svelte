<script lang="ts">
    import {t} from "svelte-i18n";
    import bgSmall from "$lib/assets/kitten_480.png";
    import bgLarge from "$lib/assets/kitten_1920.png";

    let address = "100506臺北市中正區忠孝西路一段50號20樓之32（亞洲廣場大樓）";
    let phone = "02-23707088";
    let email = "info@example.com";
</script>

<div class="contacts-us-wrapper">
    <div class="bg">
        <picture>
            <source media="(min-width: 768px)" srcset={bgLarge}>
            <img alt="" src={bgSmall}>
        </picture>
    </div>
    <div class="info">
        <div class="section">
            <div class="label">
                <i class="material-icon red">location_on</i>
                <h3>{$t('location')}</h3>
            </div>
            <p>{address}</p>
        </div>
        <div class="section">
            <div class="label">
                <i class="material-icon orange">call</i>
                <h3>{$t('call')}</h3>
            </div>
            <p><a href="tel:{phone}">{phone}</a></p>
        </div>
        <div class="section">
            <div class="label">
                <i class="material-icon blue">email</i>
                <h3>{$t('email')}</h3>
            </div>
            <p><a href="mailto:{email}">{email}</a></p>
        </div>
    </div>
</div>

<style lang="scss">
  .contacts-us-wrapper {
    width: 100%;
    padding: 1.25rem 5%;
    position: relative;

    .bg {
      width: 100%;
      display: flex;
      align-items: center;
      padding: 1.25rem 0.75rem;

      img {
        width: 100%;
      }
    }

    .label {
      display: flex;
      flex-direction: row;
      align-items: center;
      gap: 0.25rem;
    }

    .info {
      border-top: 1px solid $grey;
    }

    a {
      text-decoration: none;
      color: $black;
    }

    h3 {
      padding: 0;
    }

    p {
      padding: 0;
    }

    i {
      font-size: 1.75rem;

      &.red {
        color: $deep-red;
      }

      &.blue {
        color: $deep-blue;
      }

      &.orange {
        color: $deep-orange;
      }
    }
  }

  @media (min-width: 768px) {
    .contacts-us-wrapper {
      margin-top: 30rem;
      padding: 0;
      padding-bottom: 10rem;

      .bg {
        padding: 0;
        border-radius: 0.25rem;
        overflow: clip;
      }

      .info {
        position: absolute;
        border: none;
        background: linear-gradient(0deg, rgba(203, 223, 249, 1) 0%, rgba(255, 255, 255, 0.0) 100%);;
        backdrop-filter: blur(.25rem);
        width: 100%;
        bottom: 0;
        right: 0;
        padding: 1.25rem 10%;
        border-radius: 0.25rem;
      }
    }
  }
</style>