<script>
	import {twMerge} from "tailwind-merge";

    let className = '';
    export {className as class}
</script>

<div class={twMerge("max-w-5xl w-full flex flex-col px-2 md:px-8 ", className)}><slot/></div>