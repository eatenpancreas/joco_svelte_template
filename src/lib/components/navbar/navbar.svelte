<script lang="ts">
    import {HamburgerMenu} from "radix-icons-svelte";
    import * as Popover from "$lib/shadcn/ui/popover";
    import {onMount, setContext} from "svelte";
    import {writable} from "svelte/store";
    import {Button} from "$lib/shadcn/ui/button";
    import Hoverable from "$lib/components/Hoverable.svelte";
    import Image from "svimg/Image.svelte";

    export let title: string;
    
    let prevScrollPos = 0
    let y = 0;
    let mounted = false;
    let opened = true;
    $: scroll(y);
    
    onMount(() => {
        prevScrollPos = window.scrollY;
        y = window.scrollY;
        mounted = true;
    });
    
    function scroll(y: number) {
        if (!mounted) { return; }
        const navbar = document.getElementById("navbar");
        if (!navbar) { return; }
        
        if (prevScrollPos > y) {
            opened = true;
        } else {
            opened = false;
        }
        prevScrollPos = y;
    }


    const openStore = writable(0);
    setContext('closeNavbar', openStore);
    
    openStore.subscribe(() => {
        opened = false;
    });
    
    function open() {
        opened = true;
    }
</script>


<svelte:window bind:scrollY={y} />

<div id="navbar" 
     class="transition-all h-12 w-full bg-background flex items-center justify-between px-4 fixed z-10 border-b-border border-b
     data-[opened=true]:top-0 data-[opened=false]:top-[-64px]"
     data-opened={opened}
>
    <a href="/" class="flex items-center gap-4">
        <Image src="/favicon.png" alt="Main Icon" class="w-16"/>
        <span class="text-foreground">{title}</span>
    </a>
    <div>
        <div class="bg-primary rounded-xl items-center justify-center gap-4 hidden md:flex">
            <slot/>
        </div>
        <div class="md:hidden flex items-center">
            <Popover.Root>
                <Popover.Trigger><HamburgerMenu class="w-8 h-8 text-foreground"/></Popover.Trigger>
                <Popover.Content data-opened={opened} class="flex-col gap-4 rounded-xl 
                data-[opened=false]:hidden data-[opened=true]:flex data-[opened=true]:md:hidden">
                <slot/></Popover.Content>
            </Popover.Root>
        </div>
    </div>
</div>

{#if !opened}
    <Hoverable class="h-10 w-full fixed z-10" {open}/>
{/if}