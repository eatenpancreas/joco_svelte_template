<script lang="ts">
    import {Button} from "$lib/shadcn/ui/button/index.js";
		import { getContext, onMount } from 'svelte';
    import type {Writable} from "svelte/store";
		import { navigating } from '$app/stores';
    export let href: string;

    const close: Writable<number> = getContext('closeNavbar');
		
		onMount(() => {
			is_mounted = true;
			set_active();
		});
		
		let is_mounted = false;
		let is_active = false;

		$: if(!$navigating && is_mounted) set_active();
		
		function set_active() {
			is_active = href === window.location.pathname;
		}
    
    function closeNavbar() {
        close.set(Math.random());
    }
</script>

<Button on:click={closeNavbar} class="rounded-xl shadow-none hover:bg-white/10" href={href}>
	<div data-is-active={is_active}
		class="px-8 flex gap-2 data-[is-active=true]:border-b border-white data-[is-active=true]:text-foreground items-center">
		<slot/>
	</div>
</Button>