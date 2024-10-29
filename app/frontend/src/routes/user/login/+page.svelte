<style>
  .custom-input {
    background-color: #1D2B34;
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
  
</style>

<section class="vh-100 gradient-custom" style="background-color: #090d10;">
  <div class="container py-4 h-100">
    <div class="row d-flex justify-content-center align-items-center h-100 ">
      <div class="col-12 col-md-8 col-lg-6 col-xl-5 custom-width">
        <div class="card tw-bg-primary-dark text-white" style="border-radius: 1rem;">
          <div class="card-body p-4 pt-2 custom-height">

            <div class="mb-md-5 mt-md-4 pb-5">
              <h2 class="mb-3 tw-text-2xl">Welcome back!</h2>
              <h2 class=" mb-3 tw-text-3xl ">Sign in</h2>
              <p class="text-white-50 mb-3">Please enter your details</p>

              <div data-mdb-input-init class="form-outline  mb-4">
                <label class="form-label mt-2" for="typeEmailX">Username or Email</label>
                <input type="email" id="typeEmailX" class="form-control form-control-lg custom-input" />
                
              </div>

              <div data-mdb-input-init class="form-outline form-white mb-4">
                <label class="form-label mt-2" for="typePasswordX">Password</label>
                <input type="password" id="typePasswordX" class="form-control form-control-lg custom-input" />
                
              </div>

              <div class="remember-forgot mb-4">
                <div class="form-check">
                  <input class="form-check-input" type="checkbox" value="" id="rememberMe">
                  <label class="form-check-label" for="rememberMe">
                    Remember me
                  </label>
                </div>
                <a class="text-white-50" href="#!">Forgot password?</a>
              </div>

              <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom" type="submit">Login</button>

              <div class="center-text">
                <p class="mb-0">Don't have an account? <a href="/user/register" class="text-white-50 fw-bold">Sign Up</a></p>
              </div>

            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</section>