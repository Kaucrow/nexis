<svelte:head>
  <title>Login</title>
</svelte:head>

<script lang='ts'>
  import Checkbox from '$lib/components/Checkbox.svelte';
  import logo from '$lib/assets/Nexis.png';
  import mall from '$lib/assets/mall.jpg';

  

  let remember_me: boolean = true;

  let show_password = false;
  let type = 'password';
  
  function togglePassword() {
    show_password = !show_password;
    type = show_password ? 'text' : 'password';
  }
  
</script>

<style>

  @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap');

  .gradient-custom {
    font-family: 'Poppins', sans-serif;
  }
  .custom-input {
    background-color: #21313B;
    color: white; 
    border: none;
  }
  .custom-input::placeholder {
    color: #ccc;
  }
  .custom-width {
    max-width: 500px;
  }
  .custom-height {
    padding-top: 3rem; 
    padding-bottom: 1rem; 
  }
  .remember-forgot {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .remember-forgot .form-check {
    margin-bottom: 0;
  }
  .btn-custom {
    width: 100%; 
    background-color: #E2F1E7; 
    color: #1D2B34; 
    border: none; 
    padding: 0.75rem 1.25rem; 
    font-size: 1rem; 
    border-radius: 1.2rem; 
    margin-bottom: 2rem;
  }
  .center-text {
    text-align: center;
    margin-bottom: 0;
  }
  .full-height-image {
    position: absolute;
    height: 100vh;
    width: 30%;
    left: 0;
    top: 0;
    object-fit: cover;
  }
  .login-container {
    margin-left: 30%;
    width: 75%;
  }
  .login-card{
    border-top: 8px solid #387478;
  }
  .custom-input{
    font-size: 1.1em;
  }
  ::placeholder {
    color: #ABABAB;
    
  }
  .password-container {
    position: relative;
  }
  .toggle-password {
    position: absolute;
    right: 22px;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    cursor: pointer;
  }
  .toggle-password svg {
    width: 20px;
    height: 20px;
    filter: invert(99%) sepia(47%) saturate(217%) hue-rotate(62deg) brightness(98%) contrast(93%);
  }
</style>

<section class="vh-100 gradient-custom tw-bg-dark-background" style="position: relative;">
  <!-- svelte-ignore a11y-img-redundant-alt -->
  <img src={mall} alt="Mall Image" class="full-height-image">
  <div class="container py-4 h-100">
    <div class="row d-flex justify-content-center align-items-center h-100 ">
      <div class="col-12 col-md-8 col-lg-6 col-xl-5 custom-width login-container">
        <div class="card tw-bg-dark-card text-white login-card" style="border-radius: 25px;">
          <div class="card-body tw-px-10 p-4 pt-2 pb-1 custom-height">

            <div class="mb-md-2 mt-md-3 pb-4">
              <h3 class="mb-4 tw-text-xl  tw-text-dark-primary tw-font-poppinslight" >Welcome back!</h3>
              <h2 class=" mb-1 tw-text-2xl tw-text-dark-accent-dark tw-font-poppinsmedium">Sign in</h2>
              <p class="mb-4 tw-text-dark-primary tw-font-poppinsregular">Please enter your details</p>

              <div data-mdb-input-init class="form-outline mb-4">
                <label class="form-label mt-2 tw-text-dark-primary tw-font-poppinsregular" for="typeEmailX">Username or Email</label>
                <input type="email" id="typeEmailX" class="tw-text-dark-primary tw-rounded-lg form-control form-control-lg custom-input tw-placeholder-dark-primary" placeholder="Enter your username or email" />
              </div>

              <div data-mdb-input-init class="form-outline form-white mb-4 password-container">
                <label class="form-label mt-2 tw-text-dark-primary tw-font-poppinsregular" for="typePasswordX">Password</label>
                <div class="tw-relative custom-input tw-rounded-lg">
                  <input type={type} id="typePasswordX" class="tw-appearance-none tw-outline-none tw-text-dark-primary tw-placeholder-dark-primary tw-w-10/12 tw-border-none tw-bg-transparent form-control form-control-lg" placeholder="Enter your password" />
                  <button type="button" class="toggle-password" on:click={togglePassword}>
                    {#if show_password}
                      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M288 32c-80.8 0-145.5 36.8-192.6 80.6C48.6 156 17.3 208 2.5 243.7c-3.3 7.9-3.3 16.7 0 24.6C17.3 304 48.6 356 95.4 399.4C142.5 443.2 207.2 480 288 480s145.5-36.8 192.6-80.6c46.8-43.5 78.1-95.4 93-131.1c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C433.5 68.8 368.8 32 288 32zM144 256a144 144 0 1 1 288 0 144 144 0 1 1 -288 0zm144-64c0 35.3-28.7 64-64 64c-7.1 0-13.9-1.2-20.3-3.3c-5.5-1.8-11.9 1.6-11.7 7.4c.3 6.9 1.3 13.8 3.2 20.7c13.7 51.2 66.4 81.6 117.6 67.9s81.6-66.4 67.9-117.6c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3z"/></svg>
                    {:else}
                      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7L525.6 386.7c39.6-40.6 66.4-86.1 79.9-118.4c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C465.5 68.8 400.8 32 320 32c-68.2 0-125 26.3-169.3 60.8L38.8 5.1zM223.1 149.5C248.6 126.2 282.7 112 320 112c79.5 0 144 64.5 144 144c0 24.9-6.3 48.3-17.4 68.7L408 294.5c8.4-19.3 10.6-41.4 4.8-63.3c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3c0 10.2-2.4 19.8-6.6 28.3l-90.3-70.8zM373 389.9c-16.4 6.5-34.3 10.1-53 10.1c-79.5 0-144-64.5-144-144c0-6.9 .5-13.6 1.4-20.2L83.1 161.5C60.3 191.2 44 220.8 34.5 243.7c-3.3 7.9-3.3 16.7 0 24.6c14.9 35.7 46.2 87.7 93 131.1C174.5 443.2 239.2 480 320 480c47.8 0 89.9-12.9 126.2-32.5L373 389.9z"/></svg>
                    {/if}
                  </button>
                </div>
              </div>
              
              <!-- YET ANOTHER CHECKBOX PLS WORK OH GOD PLS WORK --> 

              <div class="tw-mb-4 tw-flex tw-items-center tw-justify-between">
                <!-- Remember me -->
                <div class="tw-flex tw-items-center tw-mb-[0.0] tw-ml-0 tw-mr-0 tw-min-h-[1.5rem] tw-ps-[0]">
                    <Checkbox bind:checked={remember_me} />
                    
                    <label
                        class="tw-font-poppinsthin tw-font-bold tw-mt-2 tw-cursor-pointer tw-text-dark-primary tw-dark:text-neutral-200 tw-relative tw-text-md tw-mx-1 tw-ps-[0.15rem] tw-bottom-[0.2rem] tw-hover:cursor-pointer tw-select-none"
                        for="Checkbox">
                        Remember me
                    </label>
                </div>

              </div>

              <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom tw-mt-3 tw-font-poppinsmedium" type="submit">Sign in</button>

              <div class="center-text">
                <p style="color:gray;" class="mb-0 tw-mt-3 ">Don't have an account? <a href="/user/register" class="fw-bold tw-text-dark-accent-dark">Sign Up</a></p>
              </div>

            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <img src={logo} alt="Nexis Logo" width="100px" height="100px" style="position: absolute; top: 10px; right: 10px;">
</section>

