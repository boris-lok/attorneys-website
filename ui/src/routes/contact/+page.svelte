<script lang="ts">
    import {t} from "svelte-i18n";

    let address = "100506臺北市中正區忠孝西路一段50號20樓之32（亞洲廣場大樓）";
    let phone = "02-23707088";
    let email = "info@example.com";
</script>

<section class="contact-us-section">
    <div class="title">{$t('contact_us')}</div>
    <div class="wrapper">
        <i class="material-icon">location_on</i>
        <span class="content">{address}</span>
    </div>
    <div class="wrapper">
        <i class="material-icon">call</i>
        <span><a href="tel:{phone}">{phone}</a></span>
    </div>
    <div class="wrapper">
        <i class="material-icon">mail</i>
        <span><a href="mailto:{email}">{email}</a></span>
    </div>
</section>

<style lang="scss">
  .contact-us-section {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    align-items: center;
    margin: 16rem 5% 5rem 5%;

    .title {
      font-size: 2rem;
      font-weight: bold;
      width: 100%;
      height: 4rem;
      line-height: 4rem;
      text-align: center;
    }

    .wrapper {
      position: relative;
      border-radius: 4px;
      width: 100%;
      padding-top: 4rem;

      a {
        text-decoration: none;
        color: $black;
      }

      span {
        font-size: 1rem;
        display: flex;
        justify-content: center;
        padding: 2.5rem 10%;
        box-shadow: 0 0 1rem 0 $grey;
      }

      i {
        position: absolute;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        overflow: clip;
        width: 4.5rem;
        height: 4.5rem;
        min-width: 4.5rem;
        min-height: 4.5rem;
        top: 0.75rem;
        left: calc(50% - 2.25rem);
        font-size: 3rem;
        color: $black;
        box-shadow: 0 0 1.5rem 0 $grey;
        background-color: $white;
      }
    }
  }

  @media (min-width: 768px) {
    .contact-us-section {
      gap: 0;
      margin: 9.5rem 10%;
      box-shadow: 0 0 1.5rem 0 $grey;
      border-radius: 0.75rem;
      padding-bottom: 2.5rem;
      overflow: clip;
      backdrop-filter: blur(.25rem);

      .title {
        width: 100%;
        height: 5rem;
        text-align: center;
        font-size: 2rem;
        font-weight: bold;
        line-height: 5rem;
        color: $white;
      }

      .wrapper {
        display: flex;
        flex-direction: row;
        gap: 1rem;
        justify-content: center;
        align-items: center;
        padding: 0 2.5rem;

        i {
          position: relative;
          top: 0;
          left: 0;
          box-shadow: none;
          font-size: 2rem;
        }

        span {
          font-size: 1.25rem;
          padding: 1.5rem 0;
          box-shadow: none;
          width: 50rem;
          justify-content: start;
        }
      }
    }
  }
</style>