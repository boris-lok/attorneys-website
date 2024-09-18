<script lang="ts">
    import {locale} from 'svelte-i18n';
    import SvelteMarkdown from "svelte-markdown";

    // Optionally, you can set the locale manually
    // $locale = 'en'; // For English
    $locale = 'zh'; // For Spanish

    let content = `# 關於本所
### 義修法律事務所，擁有多年訴訟執業與非訟辦理之經驗，專精民事訴訟、不動產爭議、公司商務爭端、稅務、商標智財、家事案件與刑事辯護等案件，致力於為客戶提供高品質之全方位法律服務，並時時掌握最新的產業趨勢與法律動態。

### 本所專業團隊提供優質之地區性及國際性訴訟，以及商標、公司設立、併購、資產規劃，盡職調查、智慧財產權、公司及勞動事務等法律協助及相關諮詢服務。我們的使命是以法律專業和全心全意服務，以協助客戶降低成本，減低風險，減免問題之發生，有效解決糾紛，維護客戶正當權益，達成企業營運目標。

### 發揮個人能力並親力親為，心繫公義同時實踐法治，是義修法律事務所最引以為傲之特色。本所將持續以最高效率與最高品質．為客戶提供周延之法律服務。`;
</script>

<section class="home-wrapper">
    <SvelteMarkdown source={content}/>
</section>

<style lang="scss">
  .home-wrapper {
    display: flex;
    flex-direction: column;
    margin: 1.5rem 5%;
  }

  @media (min-width: 768px) {
    .home-wrapper {
      margin: 3rem 10%;
      padding: 3rem;
      border-radius: 1rem;
      box-shadow: 0 0 1rem 0 $grey;
    }
  }
</style>

