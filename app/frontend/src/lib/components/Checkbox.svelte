<script lang='ts'>
    export let checked: boolean = false;
</script>

<!-- Checkbox -->
<div class="tw-relative tw-h-5 tw-w-5">
    <input 
        type="checkbox" 
        id="Checkbox"
        bind:checked 
        class="tw-peer tw-h-5 tw-w-5 tw-cursor-pointer tw-appearance-none tw-rounded tw-border-2 tw-border-neutral-500 tw-dark:border-neutral-700 tw-bg-transparent tw-transition-all tw-duration-300 tw-checked:dark:bg-neutral-700 tw-hover:border-neutral-600 tw-hover:dark:border-neutral-400 tw-checked:bg-neutral-700 tw-checked:border-neutral-700 tw-checked:dark:border-neutral-700" 
    />
    <svg 
        class="tw-pointer-events-none tw-absolute tw-inset-0 tw-m-auto tw-w-5 tw-h-5 tw-fill-white tw-stroke-white tw-scale-0 tw-transition-transform tw-duration-300 peer-checked:tw-scale-75" 
        xmlns="http://www.w3.org/2000/svg" 
        viewBox="0 0 448 512">
        <path d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"/>
    </svg>
</div>