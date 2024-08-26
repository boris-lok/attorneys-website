<script lang="ts">
    import {Router, link, Route} from 'svelte-routing';
    import Member from './lib/components/Members.svelte';
    import {faBars} from "@fortawesome/free-solid-svg-icons";

    import Fa from "svelte-fa";

    export let url = '';

    let collapsed = true;
    const toggleCollapsed = () => collapsed = !collapsed;
</script>

<main>
    <Router {url}>
        <div class="header-container">
            <nav>
                <ul class:collapsed={collapsed}>
                    <li>
                        <div class="bars" on:click={toggleCollapsed}>
                            <Fa icon={faBars} color="#ffffff"/>
                        </div>
                    </li>
                    <li class:collapsed={collapsed}>
                        <a href="/" use:link>Home</a>
                    </li>
                    <li class:collapsed={collapsed}>
                        <a href="/members" use:link>Members</a>
                    </li>
                </ul>

            </nav>
        </div>

        <div class="main-container">
            <Route path="/members" component={Member}></Route>
        </div>

        <div class="footer-container">
        </div>
    </Router>
</main>

<style lang="scss">
  @import "./constants";

  nav {
    background-color: $blue;
    box-shadow: 3px 3px 5px rgba(0, 0, 0, 0.1);
    padding: 0 16px;


    ul {
      display: flex;
      list-style: none;
      width: 100%;
      justify-content: flex-start;
      margin: 0;
      padding: 0;
      flex-direction: column;
      transition: height .4s ease-in-out;
      height: 144px;

      &.collapsed {
        height: 48px;
      }

      li {
        height: 48px;
        min-height: 48px;

        &.collapsed {
          display: none;
        }

        div.bars {
          display: flex;
          align-items: center;
          width: 32px;
          height: 100%;
          cursor: pointer;
        }

        a {
          color: $white;
          text-decoration: none;
          height: 100%;
          display: flex;
          align-items: center;
        }
      }
    }
  }

  @media only screen and (min-width: $breakpoint-md) {
    nav {
      padding: 0 48px;

      ul {
        flex-direction: row;
        column-gap: 16px;
        max-height: 48px;
        justify-content: flex-end;

        li {
          div.bars {
            display: none;
          }

          &.collapsed {
            display: block;
          }
        }
      }
    }
  }
</style>