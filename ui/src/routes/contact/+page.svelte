<script lang="ts">
    import {t} from "svelte-i18n";

    let address = "100506臺北市中正區忠孝西路一段50號20樓之32（亞洲廣場大樓）";
    let phone = "02-23707088";
    let email = "info@example.com";
</script>

<div class="contacts-us-wrapper">
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
</style>