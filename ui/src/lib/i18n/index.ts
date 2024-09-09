import {register, init, getLocaleFromNavigator} from 'svelte-i18n';

register('en', () => import('$lib/locales/en.json'));
register('zh', () => import('$lib/locales/zh.json'));

init({
    fallbackLocale: 'en',
    initialLocale: getLocaleFromNavigator() || 'en',
});