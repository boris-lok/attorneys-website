<script lang="ts">
    import {t} from "svelte-i18n";

    let isMenuOpen = false;

    function toggleMenu() {
        isMenuOpen = !isMenuOpen;
    }
</script>

<nav>
    <div class="brand">
        <span>{$t('brand')}</span>
    </div>

    <i class="material-icons menu-icon" on:click={toggleMenu}>
        {isMenuOpen ? 'close' : 'menu'}
    </i>

    <div class="nav-links" class:active={isMenuOpen}>
        <a href="/admin/members/create">{$t('members')}</a>
    </div>
</nav>

<style lang="scss">
  nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: .75rem 1.5rem;
    background-image: linear-gradient($deep-blue, $regular-blue);
    color: $white;

    .brand {
      display: flex;
      align-items: center;
      font-size: 1.5rem;
      font-weight: bold;
      line-height: 36px;
    }

    .menu-icon {
      cursor: pointer;
    }

    .nav-links {
      display: flex;
      flex-direction: column;
      position: absolute;
      top: 60px;
      left: 0;
      right: 0;
      background-image: linear-gradient($regular-blue, $light-blue);
      gap: 1.5rem;
      z-index: $layout-nav-index;
      max-height: 0;
      overflow: hidden;
      transition: max-height .25s ease-in;

      &.active {
        max-height: 100vh;
      }

      a {
        text-decoration: none;
        color: $white;
        font-weight: 500;
        transition: color 0.2s;
        padding: 1rem 1.5rem;

        &:hover {
          color: $deep-orange;
        }
      }
    }
  }

  @media (min-width: 758px) {
    nav {
      background-image: linear-gradient(to right, $deep-blue, $regular-blue);

      .nav-links {
        flex-direction: row;
        position: relative;
        max-height: 60px;
        top: 0;
        background-image: unset;
      }

      .menu-icon {
        display: none;
      }
    }
  }
</style>