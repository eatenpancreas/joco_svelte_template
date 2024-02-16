<script lang="ts">
    import "../app.css";
    import * as Navbar from "$lib/components/navbar";
    import {Person} from "radix-icons-svelte";
    import AdminOnly from '$lib/components/AdminOnly.svelte';
    import jwt from '$lib/api/jwt';
    
    let user = "Log in";

    jwt.subscribe((auth) => {
        user = (auth == undefined ? "Log in" : auth.username);
    });
</script>

<svelte:head>
    <meta name="author" content={import.meta.env.VITE_PROJECT_AUTHOR?? ""}/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <meta name="robots" content="index,follow"/>
    <meta name="theme-color" content="#fffcdc" />
    <meta name="msapplication-navbutton-color" content="#fffcdc"/>
    <meta name="apple-mobile-web-app-status-bar-style" content="#fffcdc"/>
    <meta name="apple-mobile-web-app-capable" content="yes"/>
    <meta name="mobile-web-app-capable" content="yes"/>
    <meta name="application-name" content={import.meta.env.VITE_PROJECT_TITLE?? ""}/>
    <meta name="apple-mobile-web-app-title" content={import.meta.env.VITE_PROJECT_TITLE?? ""}/>
    <meta name="msapplication-tooltip" content={import.meta.env.VITE_PROJECT_TITLE?? ""}/>
    <meta name="msapplication-starturl" content="/"/>
    <meta name="msapplication-tap-highlight" content="yes"/>
    <meta name="full-screen" content="no"/>
    <meta name="browsermode" content="application"/>
    <meta name="nightmode" content="enable/disable"/>
    <meta name="layoutmode" content="fitscreen/standard"/>
    <meta name="imagemode" content="force"/>
    <meta name="screen-orientation" content="portrait"/>
    <meta name="x5-orientation" content="portrait"/>
    <meta name="msapplication-orientation" content="portrait"/>
    <meta name="x5-page-mode" content="app"/>
</svelte:head>

<div class="bg-background from-gradient_from to-gradient_to h-screen fixed w-screen -z-50">
</div>

<div class="min-h-screen font-brevia">
    <Navbar.Root title="Svelte Template">
        <Navbar.Button href="/">Home</Navbar.Button>
        <AdminOnly><Navbar.Button href="/page/admin">Admin</Navbar.Button></AdminOnly>
        <Navbar.Button href="/page/info">Info</Navbar.Button>
        <Navbar.Button href="/page/profile"><Person/> {user}</Navbar.Button>
    </Navbar.Root>
    <div class="flex flex-col relative justify-center items-center text-foreground pb-16 pt-28">
        <slot/>
    </div>
</div>