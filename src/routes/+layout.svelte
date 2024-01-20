<script>
    import "../app.css";
    import * as Navbar from "$lib/components/navbar";
    import {Person} from "radix-icons-svelte";
    import {onMount} from "svelte";
    import {client_auth, init_auth} from "$api/auth/auth";

    onMount(() => {
        init_auth();
    });
    
    let user = "Guest";
    
    client_auth.subscribe((auth) => {
        if (auth != null) { user = auth.username; }
        else { user = "Guest"; }
    });
</script>

<div class="bg-gradient-to-tl from-gradient_from to-gradient_to h-screen fixed w-screen -z-50"></div>

<div class="min-h-screen font-brevia">
    <Navbar.Root title="Svelte Template">
        <Navbar.Button href="/info">Info</Navbar.Button>
        <Navbar.Button href="/profile"><Person/> {user}</Navbar.Button>
    </Navbar.Root>
    <div class="flex flex-col relative justify-center items-center text-foreground pb-16 pt-28">
        <slot/>
    </div>
</div>