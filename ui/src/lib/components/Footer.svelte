<script lang="ts">
    import {t} from "svelte-i18n";
</script>

<footer>
    <section>
        <h3>{$t('about_us')}</h3>
        <p>{$t('about_us.content')}</p>
    </section>

    <section>
        <h3>{$t('contact_us')}</h3>
        <p>{$t('phone')}: 02-23707088 </p>
        <p>{$t('address')}: 100506臺北市中正區忠孝西路一段50號20樓之32（亞洲廣場大樓） </p>
    </section>
</footer>

<style lang="scss">
  footer {
    display: flex;
    padding: 1rem 1.5rem;
    color: $white;
    background-color: $deep-blue;
    justify-content: center;
    flex-direction: column;
    align-items: center;
    text-align: center;

    section {

      h3 {
        margin-bottom: .5rem;
        font-size: 1.2rem;
        font-weight: bold;
      }

      p {
        margin: .5rem;
        font-size: 0.8rem;
        font-weight: 500;
      }
    }
  }
</style>