<script>
    import "../app.css";
    import { Button } from "$lib/shadcn/ui/button";
</script>


<div class="bg-background h-screen fixed w-screen -z-50"></div>

<div class="min-h-screen">
    <div class="h-12 w-full bg-accent flex items-center justify-between px-4">
        <div class="flex items-center gap-4">
            <img src="favicon.png" alt="Main Icon" class="w-16">
            <span class="text-foreground">
                Minecraft Server
            </span>
        </div>
        <div class="bg-primary rounded-2xl flex items-center justify-center gap-4">
            <Button class="rounded-2xl px-8 shadow-none hover:bg-white/10" href="/">Home</Button>
            <Button class="rounded-2xl px-8 shadow-none hover:bg-white/10" href="/admin">Admin</Button>
            <Button class="rounded-2xl px-8 shadow-none hover:bg-white/10" href="/info">Info</Button>
            <Button class="rounded-2xl px-8 shadow-none hover:bg-white/10" href="/profile">Profile</Button>
        </div>
    </div>
    <div class="flex flex-col relative justify-center items-center text-foreground py-16">
        <slot/>
    </div>
</div>