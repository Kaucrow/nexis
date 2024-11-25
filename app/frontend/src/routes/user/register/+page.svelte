<svelte:head>
  <title>Register</title>
</svelte:head>

<script lang="ts">
  import { writable } from 'svelte/store';
  import logo from '$lib/assets/Nexis.png';
  import mall2 from '$lib/assets/mall2-2.jpg'
  import Radio from '$lib/components/Radio.svelte';
  import SecureInput from '$lib/components/SecureInput.svelte';

  let selectedLocal = writable('');
  
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
  .form-container {
    transform: scale(0.9);
    transform-origin: center;
  }
  .btn-custom {
    width: 80%; 
    background-color: #E2F1E7; 
    color: #1D2B34; 
    border: none; 
    padding: 0.75rem 1.25rem; 
    font-size: 1rem; 
    border-radius: 2rem; 
    margin-bottom: 2rem;
  }
  .custom-input {
    background-color: #21313B;
    color: white;
    border: none; 
  }
  .full-height-image {
    position: absolute;
    height: 100vh;
    width: 28%;
    right: 0;
    top: 0;
    object-fit: cover;
  }
  .form-container {
    margin-right: 35%;
    width: 75%;
  }
  .login-card{
    border-top: 10px solid #387478;
  }
  ::placeholder {
    color: #ccc;
    opacity: 1;
    
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

<section class="vh-100 gradient-custom " style="background-color: #131A20">
  <img src={mall2} alt="Mall Image" class="full-height-image">
    <div class="container py-1 h-100">
      <div class="row justify-content-center align-items-center h-100">
        <div class="col-12 col-lg-9 col-xl-7 form-container">
          <div class="card shadow-2-strong card-registration tw-bg-dark-card text-white login-card" style="border-radius: 30px;">
            <div class="card-body p-4 p-md-5 ">
              <h3 class="tw-text-xl pb-2 tw-text-dark-primary">Welcome !</h3>
              <h3 class="mb-2 pb-4 pb-md-0 mb-md-4 tw-text-3xl tw-text-dark-primary">Sign up to Nexis</h3>
              <form>

                <div class="row tw-mt-4">
                  <div class="col-md-6 mb-4">
  
                    <div data-mdb-input-init class="form-outline">
                      <label class="form-label tw-text-dark-primary" for="emailAddress">Email</label>
                      <input type="email" id="emailAddress" class="form-control form-control-lg custom-input" placeholder="Enter your email"/>
                    </div>
  
                  </div>
                  <div class="col-md-6 mb-4 ">
  
                    <div data-mdb-input-init class="form-outline">
                      <label class="form-label tw-text-dark-primary" for="phoneNumber">Phone Number</label>
                      <input type="tel" id="phoneNumber" class="form-control form-control-lg custom-input" placeholder="Enter your phone number" />
                    </div>
  
                  </div>
                </div>
  
                <div class="row">
                  <div class="col-md-6 mb-4">
  
                    <div data-mdb-input-init class="form-outline ">
                      <label class="form-label tw-text-dark-primary" for="firstName">Username</label>
                      <input type="text" id="firstName" class="form-control form-control-lg custom-input" placeholder="Enter your username" />
                      
                    </div>
  
                  </div>

                  <div class="col-md-6 mb-3">
  
                    <h6 class="mb-2 pb-3 tw-text-dark-primary">Gender: </h6>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="femaleGender"
                        value="option1" checked />
                      <label class="form-check-label tw-text-dark-primary" for="femaleGender">Female</label>
                    </div>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="maleGender"
                        value="option2" />
                      <label class="form-check-label tw-text-dark-primary" for="maleGender">Male</label>
                    </div>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="otherGender"
                        value="option3" />
                      <label class="form-check-label tw-text-dark-primary" for="otherGender">Other</label>
                    </div>
                  </div>
                  
                </div>

                <SecureInput label="Intento de explicacion del atla" placeholder="Enter your password" />
                <SecureInput label="Llorando por dentro" placeholder="Enter your password" />

                <!-- Row 3 -->
                <div class="row">

                  <!-- Password -->
                  <div class="col-md-6 mb-4 d-flex align-items-center">
                    <div data-mdb-input-init class="tw-relative tw-form-outline form-white w-100 tw-bg-dark-shade-light">
                      <input type={type} id="typePasswordX" class="tw-appearance-none tw-outline-none tw-text-dark-primary tw-w-11/12 tw-border-none tw-bg-transparent form-control form-control-lg" placeholder="Enter your password" />
                      <button type="button" class="toggle-password" on:click={togglePassword}>
                        {#if show_password}
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M288 32c-80.8 0-145.5 36.8-192.6 80.6C48.6 156 17.3 208 2.5 243.7c-3.3 7.9-3.3 16.7 0 24.6C17.3 304 48.6 356 95.4 399.4C142.5 443.2 207.2 480 288 480s145.5-36.8 192.6-80.6c46.8-43.5 78.1-95.4 93-131.1c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C433.5 68.8 368.8 32 288 32zM144 256a144 144 0 1 1 288 0 144 144 0 1 1 -288 0zm144-64c0 35.3-28.7 64-64 64c-7.1 0-13.9-1.2-20.3-3.3c-5.5-1.8-11.9 1.6-11.7 7.4c.3 6.9 1.3 13.8 3.2 20.7c13.7 51.2 66.4 81.6 117.6 67.9s81.6-66.4 67.9-117.6c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3z"/></svg>
                        {:else}
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7L525.6 386.7c39.6-40.6 66.4-86.1 79.9-118.4c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C465.5 68.8 400.8 32 320 32c-68.2 0-125 26.3-169.3 60.8L38.8 5.1zM223.1 149.5C248.6 126.2 282.7 112 320 112c79.5 0 144 64.5 144 144c0 24.9-6.3 48.3-17.4 68.7L408 294.5c8.4-19.3 10.6-41.4 4.8-63.3c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3c0 10.2-2.4 19.8-6.6 28.3l-90.3-70.8zM373 389.9c-16.4 6.5-34.3 10.1-53 10.1c-79.5 0-144-64.5-144-144c0-6.9 .5-13.6 1.4-20.2L83.1 161.5C60.3 191.2 44 220.8 34.5 243.7c-3.3 7.9-3.3 16.7 0 24.6c14.9 35.7 46.2 87.7 93 131.1C174.5 443.2 239.2 480 320 480c47.8 0 89.9-12.9 126.2-32.5L373 389.9z"/></svg>
                        {/if}
                      </button>
                    </div>
                  </div>

                  <!-- Password (poggers) -->
                  <div data-mdb-input-init class="form-outline form-white mb-4 password-container">
                    <label class="form-label mt-2 tw-text-dark-primary" for="typePasswordX">Password</label>
                    <div class="tw-relative custom-input tw-rounded-lg">
                      <input type={type} id="typePasswordX" class="tw-appearance-none tw-outline-none tw-text-dark-primary tw-w-11/12 tw-border-none tw-bg-transparent form-control form-control-lg" placeholder="Enter your password" />
                      <button type="button" class="toggle-password" on:click={togglePassword}>
                        {#if show_password}
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 576 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M288 32c-80.8 0-145.5 36.8-192.6 80.6C48.6 156 17.3 208 2.5 243.7c-3.3 7.9-3.3 16.7 0 24.6C17.3 304 48.6 356 95.4 399.4C142.5 443.2 207.2 480 288 480s145.5-36.8 192.6-80.6c46.8-43.5 78.1-95.4 93-131.1c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C433.5 68.8 368.8 32 288 32zM144 256a144 144 0 1 1 288 0 144 144 0 1 1 -288 0zm144-64c0 35.3-28.7 64-64 64c-7.1 0-13.9-1.2-20.3-3.3c-5.5-1.8-11.9 1.6-11.7 7.4c.3 6.9 1.3 13.8 3.2 20.7c13.7 51.2 66.4 81.6 117.6 67.9s81.6-66.4 67.9-117.6c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3z"/></svg>
                        {:else}
                          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M38.8 5.1C28.4-3.1 13.3-1.2 5.1 9.2S-1.2 34.7 9.2 42.9l592 464c10.4 8.2 25.5 6.3 33.7-4.1s6.3-25.5-4.1-33.7L525.6 386.7c39.6-40.6 66.4-86.1 79.9-118.4c3.3-7.9 3.3-16.7 0-24.6c-14.9-35.7-46.2-87.7-93-131.1C465.5 68.8 400.8 32 320 32c-68.2 0-125 26.3-169.3 60.8L38.8 5.1zM223.1 149.5C248.6 126.2 282.7 112 320 112c79.5 0 144 64.5 144 144c0 24.9-6.3 48.3-17.4 68.7L408 294.5c8.4-19.3 10.6-41.4 4.8-63.3c-11.1-41.5-47.8-69.4-88.6-71.1c-5.8-.2-9.2 6.1-7.4 11.7c2.1 6.4 3.3 13.2 3.3 20.3c0 10.2-2.4 19.8-6.6 28.3l-90.3-70.8zM373 389.9c-16.4 6.5-34.3 10.1-53 10.1c-79.5 0-144-64.5-144-144c0-6.9 .5-13.6 1.4-20.2L83.1 161.5C60.3 191.2 44 220.8 34.5 243.7c-3.3 7.9-3.3 16.7 0 24.6c14.9 35.7 46.2 87.7 93 131.1C174.5 443.2 239.2 480 320 480c47.8 0 89.9-12.9 126.2-32.5L373 389.9z"/></svg>
                        {/if}
                      </button>
                    </div>
                  </div>

                  <!-- Birth Date -->
                  <div class="col-md-6 mb-4 d-flex align-items-center">
                    <div data-mdb-input-init class="form-outline datepicker w-100">
                      <label for="birthdayDate" class="form-label tw-text-dark-primary">Birth date (MM/DD/YY)</label>
                      <input type="text" class="form-control form-control-lg custom-input" id="birthdayDate" placeholder="Enter your birth date" />
                    </div>
                  </div>

                </div>
                
                <div class="row">
                  <div class="col-md-6 mb-4 d-flex align-items-center">
                    <div data-mdb-input-init class="form-outline form-white w-100">
                      <label class="form-label tw-text-dark-primary" for="typeRepeatPasswordX">Repeat Password</label>
                      <input type="password" id="typeRepeatPasswordX" class="form-control form-control-lg custom-input" placeholder="Repeat your password"/>
                    </div>
                  </div>
                </div>
                
                <div class="mt-3 d-flex justify-content-center align-items-center">
                  <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom fw-bold" type="submit">Sign up</button>
                </div>

                <div class=" d-flex justify-content-center align-items-center">
                    <p style="color:gray;" class="mt-2 mb-0">Already have an account? <a href="/user/login" class="text-white-50 fw-bold tw-text-dark-primary">Sign In</a></p>
                </div>
  
              </form>
              
            </div>
          </div>
        </div>
      </div>
    </div>
    <img src={logo} alt="Nexis Logo" width="100px" height="100px" style="position: absolute; top: 10px; left: 10px;">
  </section>