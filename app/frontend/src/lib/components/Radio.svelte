<script lang='ts'>
//    let className = '';
//    export { className as class };
    import type { Writable } from "svelte/store";
    export let selectedComponent: Writable<string>;
    export let name;
    export let value;
</script>

<div class="tw-cursor-pointer tw-border-[0.2em] tw-border-dark-accent-light tw-relative tw-rounded-full tw-h-5 tw-w-5">
    <input 
        type="radio" 
        id="Radio"
        name={name}
        value={value}
        bind:group={selectedComponent}
        class="tw-peer tw-rounded-full tw-bg-transparent tw-h-5 tw-w-5 tw-appearance-none" 
    />
    <div class="tw-top-1/2 tw-left-1/2 tw-transform tw-translate-x-[-50%] tw-translate-y-[-50%] tw-pointer-events-none tw-absolute tw-h-2 tw-w-2 tw-bg-dark-accent-light tw-rounded-full tw-transition-transform tw-duration-300 tw-scale-0 peer-checked:tw-scale-100"></div>
</div>