<script lang="ts">
    import {t, locale} from 'svelte-i18n';

    // Optionally, you can set the locale manually
    // $locale = 'en'; // For English
    $locale = 'zh'; // For Spanish
</script>

<h1>{$t('welcome')} to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
