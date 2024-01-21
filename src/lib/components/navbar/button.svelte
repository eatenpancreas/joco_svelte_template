<script lang="ts">
    import {Button} from "$lib/shadcn/ui/button/index.js";
    import {getContext} from "svelte";
    import type {Writable} from "svelte/store";
    export let href: string;

    const close: Writable<number> = getContext('closeNavbar');
    
    function closeNavbar() {
        close.set(Math.random());
    }
</script>

<Button on:click={closeNavbar} class="rounded-xl px-8 shadow-none hover:bg-white/10 flex gap-2" href={href}><slot/></Button>