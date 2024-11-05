<svelte:head>
  <title>Register</title>
</svelte:head>

<script lang="ts">
  import { writable } from 'svelte/store';
  import logo from '$lib/assets/Nexis.png';
  import mall2 from '$lib/assets/mall2-2.jpg'
  import Radio from '$lib/components/Radio.svelte';

  let selectedLocal = writable('');
</script>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap');

.gradient-custom {
  font-family: 'Poppins', sans-serif;
}
  .form-container {
    transform: scale(0.8);
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
    background-color: #1D2B34;
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
    margin-right: 30%;
    width: 75%;
  }
  .login-card{
    border-top: 10px solid #387478;
  }
  ::placeholder {
    color: #ccc;
    opacity: 1;
    
  }

</style>

<section class="vh-100 gradient-custom " style="background-color: #131A20">
  <img src={mall2} alt="Mall Image" class="full-height-image">
    <div class="container py-1 h-100">
      <div class="row justify-content-center align-items-center h-100">
        <div class="col-12 col-lg-9 col-xl-7 form-container">
          <div class="card shadow-2-strong card-registration tw-bg-dark-shade-light text-white login-card" style="border-radius: 30px;">
            <div class="card-body p-4 p-md-5 ">
              <h3 class="tw-text-xl pb-2 tw-text-primary-light">Welcome !</h3>
              <h3 class="mb-2 pb-4 pb-md-0 mb-md-4 tw-text-3xl tw-text-primary-secondary">Sign up to Nexis</h3>
              <form>

                <div class="row">
                  <div class="col-md-6 mb-4">
  
                    <div data-mdb-input-init class="form-outline">
                      <label class="form-label tw-text-primary-light" for="emailAddress">Email</label>
                      <input type="email" id="emailAddress" class="form-control form-control-lg custom-input" placeholder="Enter your email"/>
                    </div>
  
                  </div>
                  <div class="col-md-6 mb-4 ">
  
                    <div data-mdb-input-init class="form-outline">
                      <label class="form-label tw-text-primary-light" for="phoneNumber">Phone Number</label>
                      <input type="tel" id="phoneNumber" class="form-control form-control-lg custom-input" placeholder="Enter your phone number" />
                      
                    </div>
  
                  </div>
                </div>
  
                <div class="row">
                  <div class="col-md-6 mb-4">
  
                    <div data-mdb-input-init class="form-outline ">
                      <label class="form-label tw-text-primary-light" for="firstName">Username</label>
                      <input type="text" id="firstName" class="form-control form-control-lg custom-input" placeholder="Enter your username" />
                      
                    </div>
  
                  </div>

                  <div class="col-md-6 mb-3">
  
                    <h6 class="mb-2 pb-3 tw-text-primary-light">Gender: </h6>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="femaleGender"
                        value="option1" checked />
                      <label class="form-check-label tw-text-primary-light" for="femaleGender">Female</label>
                    </div>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="maleGender"
                        value="option2" />
                      <label class="form-check-label tw-text-primary-light" for="maleGender">Male</label>
                    </div>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="otherGender"
                        value="option3" />
                      <label class="form-check-label tw-text-primary-light" for="otherGender">Other</label>
                    </div>

                    <Radio name="example" value="option1" selectedComponent={selectedLocal}/>
                    <Radio name="example" value="option2" selectedComponent={selectedLocal}/>
                    <Radio name="example" value="option3" selectedComponent={selectedLocal}/>

                    <p class="tw-bg-slate-900 tw-text-slate-100">Selected: {$selectedLocal}</p>
                  </div>
                  
                </div>

                <div class="row">
                  <div class="col-md-6 mb-4 d-flex align-items-center">
                    <div data-mdb-input-init class="form-outline form-white w-100">
                      <label class="form-label tw-text-primary-light" for="typePasswordX">Password</label>
                      <input type="password" id="typePasswordX" class="form-control form-control-lg custom-input" placeholder="Enter your password" />
                    </div>
                  </div>
                  <div class="col-md-6 mb-4 d-flex align-items-center">
                    <div data-mdb-input-init class="form-outline datepicker w-100">
                      <label for="birthdayDate" class="form-label tw-text-primary-light">Birth date (MM/DD/YY)</label>
                      <input type="text" class="form-control form-control-lg custom-input" id="birthdayDate" placeholder="Enter your birth date" />
                    </div>
                  </div>
                </div>
                
                <div class="row">
                  <div class="col-md-6 mb-4 d-flex align-items-center">
                    <div data-mdb-input-init class="form-outline form-white w-100">
                      <label class="form-label tw-text-primary-light" for="typeRepeatPasswordX">Repeat Password</label>
                      <input type="password" id="typeRepeatPasswordX" class="form-control form-control-lg custom-input" placeholder="Repeat your password"/>
                    </div>
                  </div>
                </div>
                
                <div class="mt-3 d-flex justify-content-center align-items-center">
                  <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom fw-bold" type="submit">Sign up</button>
                </div>

                <div class=" d-flex justify-content-center align-items-center">
                    <p style="color:gray;" class="mt-2 mb-0">Already have an account? <a href="/user/login" class="text-white-50 fw-bold tw-text-primary-secondary">Sign In</a></p>
                </div>
  
              </form>
              
            </div>
          </div>
        </div>
      </div>
    </div>
    <img src={logo} alt="Nexis Logo" width="100px" height="100px" style="position: absolute; top: 10px; left: 10px;">
  </section>