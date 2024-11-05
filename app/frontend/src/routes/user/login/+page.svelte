<svelte:head>
  <title>Login</title>
</svelte:head>

<script lang='ts'>
  import Checkbox from '$lib/components/Checkbox.svelte';
  import logo from '$lib/assets/Nexis.png';
  import mall from '$lib/assets/mall.jpg';

  let show_password: boolean = true;
  $: type = show_password ? 'text' : 'password';

  let remember_me: boolean = true;
  
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
    border-radius: 2rem; 
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
    color: #ccc;
    opacity: 1;
  }
  
</style>

<section class="vh-100 gradient-custom tw-bg-dark-background" style="position: relative;">
  <!-- svelte-ignore a11y-img-redundant-alt -->
  <img src={mall} alt="Mall Image" class="full-height-image">
  <div class="container py-4 h-100">
    <div class="row d-flex justify-content-center align-items-center h-100 ">
      <div class="col-12 col-md-8 col-lg-6 col-xl-5 custom-width login-container">
        <div class="card tw-bg-primary-dark text-white login-card" style="border-radius: 25px;">
          <div class="card-body tw-px-10 p-4 pt-2 pb-1 custom-height">

            <div class="mb-md-2 mt-md-3 pb-4">
              <h3 class="mb-4 tw-text-xl  tw-text-primary-light" >Welcome back!</h3>
              <h2 class=" mb-1 tw-text-2xl tw-text-primary-secondary">Sign in</h2>
              <p class="mb-4 tw-text-primary-light">Please enter your details</p>

              <div data-mdb-input-init class="form-outline  mb-4">
                <label class="form-label mt-2 tw-text-primary-light" for="typeEmailX">Username or Email</label>
                <input type="email" id="typeEmailX" class="form-control form-control-lg custom-input" placeholder="Enter your username or email" />
              </div>

              <div data-mdb-input-init class="form-outline form-white mb-4">
                <label class="form-label mt-2 tw-text-primary-light" for="typePasswordX">Password</label>
                <input {type} id="typePasswordX" class="form-control form-control-lg custom-input" placeholder="Enter your password" />
                <button type="button" on:click="{ () => show_password = !show_password }">
                  {#if show_password}
                    <div class="tw-text-neutral-800">FDAHFDSHFHDSLKHFKJSLDFLHSHREWHRJKDHFSLKHERKJWHRLKEWHKLEWHFKHSLKHDS</div>

                    <i class="fas fa-eye-slash"></i>
                  {:else}
                    <div class="tw-text-neutral-800">FDAHFDSHFHDSLKHFK</div>
                    <i class="fas fa-eye"></i>
                  {/if}
                </button>
                
              </div>
              
              <!-- YET ANOTHER CHECKBOX PLS WORK OH GOD PLS WORK --> 

              <div class="tw-mb-0 tw-flex tw-items-center tw-justify-between">
                <!-- Remember me -->
                <div class="tw-flex tw-items-center tw-mb-[0.0] tw-ml-0 tw-mr-0 tw-min-h-[1.5rem] tw-ps-[0]">
                    <Checkbox bind:checked={remember_me} />
                    <label
                        class=" tw-cursor-pointer tw-text-neutral-700 tw-dark:text-neutral-200 tw-relative tw-text-md tw-mx-1 tw-ps-[0.15rem] tw-bottom-[0.2rem] tw-hover:cursor-pointer tw-select-none"
                        for="Checkbox">
                        Remember me
                    </label>
                </div>
              </div>

              <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom fw-bold" type="submit">Sign in</button>

              <div class="center-text">
                <p style="color:gray;" class="mb-0">Don't have an account? <a href="/user/register" class="fw-bold tw-text-primary-secondary">Sign Up</a></p>
              </div>

            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <img src={logo} alt="Nexis Logo" width="100px" height="100px" style="position: absolute; top: 10px; right: 10px;">
</section>

