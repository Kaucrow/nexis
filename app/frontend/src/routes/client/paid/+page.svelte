<svelte:head>
    <title>Paid</title>
</svelte:head>

<script lang="ts">
    import logo from '$lib/assets/Nexis.png';

    
</script>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap');

    :global(body) {
        font-family: 'Poppins', sans-serif;
        background: #0D1317;
        margin: 0;
        padding: 0;
        overflow: hidden;
    }

</style>

<div class="tw-min-h-screen tw-p-14 tw-bg-[#0D1317]">
    <p class="tw-text-[#E2F1E7]">Nah que pro pana si pagaste</p>
    <p class="tw-text-[#E2F1E7]">Gracias por pagarnos la universidad</p>
    <img src={logo} alt="Nexis Logo" class="tw-fixed tw-top-14 tw-right-14 tw-w-12 tw-h-12">
</div>

