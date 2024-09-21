<script lang="ts">
	import { locale, t } from 'svelte-i18n';
	import bgSmall from '$lib/assets/justice_480.png';
	import bgLarge from '$lib/assets/justice_1920.png';
	import { page } from '$app/stores';

	let isMenuOpen = false;
	let width: number;
	let path = '/';

	// handles the menu/close icon clicked
	function toggleMenu() {
		isMenuOpen = !isMenuOpen;
	}

	// handles the path changed
	function onPathChanged(newPath: string) {
		isMenuOpen = false;
		console.log(`new path: ${newPath}`);
		path = newPath;
	}

	// TODO: support multiple languages
	$locale = 'zh';

	// listen the window's width changed
	$: if (width > 768) {
		isMenuOpen = false;
	}

	// listen the path changed
	$: onPathChanged($page.url.pathname);
</script>

<svelte:window bind:innerWidth={width} on:touchmove|nonpassive={e => {
	if (isMenuOpen) {e.preventDefault();}
}} on:wheel|nonpassive={e => {
    if (isMenuOpen) {e.preventDefault();}
}} />
<header class="header">
	<div class="top-bar">
		<a class="logo" href="/">Logo</a>
		<div class="icons">
			<button class="material-icon hidden" class:active={!isMenuOpen} on:click={toggleMenu}>menu</button>
			<button class="material-icon hidden" class:active={isMenuOpen} on:click={toggleMenu}>close</button>
		</div>

		<nav class="navbar" class:active={isMenuOpen}>
			<a class:active={path === '/'} href="/">
				<span class="material-icon">home</span>
				<span>{$t('navbar.home')}</span>
			</a>
			<a class:active={path === '/services'} href="/services">
				<span class="material-icon">event_note</span>
				<span>{$t('navbar.services')}</span>
			</a>
			<a class:active={path === '/articles'} href="/articles">
				<span class="material-icon">menu_book</span>
				<span>{$t('navbar.articles')}</span>
			</a>
			<a class:active={path === '/members'} href="/members">
				<span class="material-icon">group</span>
				<span>{$t('navbar.members')}</span>
			</a>
			<a class:active={path==='/contact'} href="/contact">
				<span class="material-icon">contacts</span>
				<span>{$t('navbar.contact_us')}</span>
			</a>
		</nav>
	</div>
</header>
<section class="bg">
	<picture>
		<source media="(min-width: 768px)" srcset={bgLarge}>
		<img alt="" src={bgSmall}>
	</picture>
</section>

<style lang="scss">
  header {
    position: relative;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-shadow: 0 0 0.25rem 0 $deep-grey;
    z-index: $layout-nav-index;


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
        color: $black;
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

        &.active {
          color: $deep-orange;
          cursor: default;
          pointer-events: none;
        }

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

    .bg {
      img {
        height: 20rem;
        object-fit: cover;
      }
    }
  }
</style>