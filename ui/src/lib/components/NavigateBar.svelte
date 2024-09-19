<script lang="ts">
    import {locale, t} from "svelte-i18n";
    import bgSmall from "$lib/assets/justice_480.png";
    import bgLarge from "$lib/assets/justice_1280.png";

    let isMenuOpen = false;
    let width: number;

    function toggleMenu() {
        isMenuOpen = !isMenuOpen;
    }

    // TODO: support multiple languages
    $locale = 'zh';

    $: if (width > 768) {
        isMenuOpen = false;
    }
</script>

<svelte:window bind:innerWidth={width} on:wheel|nonpassive={e => {
    if (isMenuOpen) {e.preventDefault();}
}}/>
<header class="header">
    <div class="top-bar">
        <a class="logo" href="#">Logo</a>
        <label class="icons" for="">
            <button class="material-icon hidden" class:active={!isMenuOpen} on:click={toggleMenu}>menu</button>
            <button class="material-icon hidden" class:active={isMenuOpen} on:click={toggleMenu}>close</button>
        </label>

        <nav class="navbar" class:active={isMenuOpen}>
            <a href="#">
                <span class="material-icon">home</span>
                <span>{$t('navbar.home')}</span>
            </a>
            <a href="#">
                <span class="material-icon">event_note</span>
                <span>{$t('navbar.services')}</span>
            </a>
            <a href="#">
                <span class="material-icon">group</span>
                <span>{$t('navbar.members')}</span>
            </a>
            <a href="#">
                <span class="material-icon">contacts</span>
                <span>{$t('navbar.contact_us')}</span>
            </a>
        </nav>
    </div>

    <section class="bg">
        <picture>
            <source media="(min-width: 768px)" srcset={bgLarge}>
            <img alt="bg-image" src={bgSmall}>
        </picture>
    </section>

</header>

<style lang="scss">
  header {
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 0 0.25rem 0 $deep-grey;

    .bg {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      z-index: -1;
      overflow-clip: clip;
      width: 100%;

      img {
        width: 100%;
        height: 320px;
      }
    }

    .top-bar {
      display: flex;
      flex-direction: row;
      padding: 1.25rem 5%;
      align-items: center;
      justify-content: space-between;
      width: 100%;
      background-color: rgba(248, 250, 252, 0.6);
      backdrop-filter: blur(.25rem);

      .logo {
        font-size: 2rem;
        font-weight: bold;
      }
    }

    .icons {
      .hidden {
        display: none;
      }

      .active {
        display: inline-block;
      }

      button {
        font-size: 2rem;
        cursor: pointer;
        background-color: transparent;
        border: none;
        outline: none;
      }
    }

    .navbar {
      position: absolute;
      top: 100%;
      left: 0;
      right: 0;
      flex-direction: column;
      height: 0;
      overflow: clip;
      transition: all 0.5s cubic-bezier(.77, 0, .18, 1);
      z-index: $layout-nav-index;
      display: flex;

      &.active {
        height: calc(100vh - 90px);
        overflow-y: hidden;
        backdrop-filter: blur(1rem);
      }

      a {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 0.5rem;
        margin-top: 2rem;
        margin-left: 0.5rem;
        position: relative;
        padding: 0 1.25rem;

        span:nth-child(1) {
          font-size: 1.5rem;
          position: absolute;
          top: 0.5rem;
        }

        span:nth-child(2) {
          font-size: 1.5rem;
          font-weight: 500;
          display: inline-block;
          margin-left: 2rem;
        }
      }
    }
  }

  a {
    color: $black;
    text-decoration: none;
  }

  @media (min-width: 768px) {
    .header {
      .icons {
        display: none;
      }

      .navbar {
        position: relative;
        height: fit-content;
        display: flex;
        flex-direction: row;

        &.active {
          height: fit-content;
        }

        a {
          transition: color 0.3s ease-in-out;

          &:hover {
            color: $deep-blue;
          }

          span:nth-child(1) {
            position: relative;
            top: 0;
          }

          span:nth-child(2) {
            font-size: 1rem;
            margin-left: 0.25rem;
            white-space: nowrap;
          }
        }
      }
    }
  }
</style>