<script lang="ts">
import {twMerge} from "tailwind-merge";

let className = "";

export {className as class}
</script>
<div class={twMerge("bg-card border-border border text-foreground p-4 pb-16 pt-20 rounded-xl shadow-xl shadow-black/40", className)}>
    <slot/>
</div>