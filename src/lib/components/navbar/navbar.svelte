<script lang="ts">
    export let title: string;
</script>

<div class="h-12 w-full bg-accent flex items-center justify-between px-4 shadow-xl shadow-black/40 relative     z-10">
    <a href="/" class="flex items-center gap-4">
        <img src="favicon.png" alt="Main Icon" class="w-16">
        <span class="text-foreground">{title}</span>
    </a>
    <div class="bg-primary rounded-2xl flex items-center justify-center gap-4">
        <slot/>
    </div>
</div>