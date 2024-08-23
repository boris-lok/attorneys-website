<script>
    import {Router, Link, Route} from 'svelte-routing';
    import Member from './lib/components/member.svelte'

    export let url = '';
</script>

<Router {url}>
    <nav>
        <Link to="/">Home</Link>
        <Link to="/member">Member List</Link>
    </nav>
    <div>
        <Route path="/member" component={Member}></Route>
    </div>
</Router>