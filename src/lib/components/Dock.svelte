<script lang="ts">
import {twMerge} from "tailwind-merge";

let className = "";

export {className as class}
</script>
<div class={twMerge("bg-secondary p-4 pb-16 pt-20 rounded-2xl shadow-xl shadow-black/40", className)}>
    <slot/>
</div>