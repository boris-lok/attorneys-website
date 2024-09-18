<script lang="ts">
    import {locale} from 'svelte-i18n';
    import SvelteMarkdown from "svelte-markdown";

    // Optionally, you can set the locale manually
    // $locale = 'en'; // For English
    $locale = 'zh'; // For Spanish

    let content = `# 服務項目

## 訴訟及爭端處理
- 民事、刑事訴訟
- 行政訴訟 / 國家賠償
- 促參、政府採購訴訟/調解
- 強制執行

## 非訟事件
- 盡職調查Due Diligent
- 各式合約撰寫／翻譯
- 見證服務
- 公司設立登記

## 智慧財產權保護
- 商標申請、商標異議、陳述意見及相關訴訟
- 著作權維權、
- 公平交易案件
- 國際網域名稱保護
- 個人資料保護案件
- 專利檢索、專利申請、專利侵害分析報告、公司智財輔導

## 公司及投資
- 證券及資本市場、保險法
- 促參案件、工程及政府採購（含軍事採購）
- 銀行與融資
- 企業合併及股權收購
- 稅務案件
- 不動產／營建案件
- 勞資糾紛
    `;
</script>

<section class="services-wrapper">
    <SvelteMarkdown source={content}/>
</section>

<style lang="scss">
  .services-wrapper {
    display: flex;
    flex-direction: column;
    margin: 1.5rem 5%;
  }

  @media (min-width: 768px) {
    .services-wrapper {
      margin: 3rem 10%;
      padding: 3rem;
      border-radius: 1rem;
      box-shadow: 0 0 1rem 0 $grey;
    }
  }
</style>

