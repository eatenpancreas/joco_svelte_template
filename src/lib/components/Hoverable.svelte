<script lang="ts">
    export let open: () => void;

    let className = "";

    export {className as class}
</script>

<button on:click={open} class={className} on:mouseenter={open} name="popup field"
>
    <slot/>
</button>