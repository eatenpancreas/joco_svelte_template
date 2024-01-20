<script lang="ts">
    import {Button} from "$lib/shadcn/ui/button/index.js";
    export let href: string;
</script>

<Button class="rounded-2xl px-8 shadow-none hover:bg-white/10 flex gap-2" href={href}><slot/></Button>