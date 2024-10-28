
<section class="vh-100 gradient-custom" style="background-color: #090d10;">
    <div class="container py-4 h-100">
      <div class="row d-flex justify-content-center align-items-center h-100">
        <div class="col-12 col-md-8 col-lg-6 col-xl-5">
          <div class="card tw-bg-primary-dark text-white" style="border-radius: 1rem;">
            <div class="card-body p-4 text-center">
  
              <div class="mb-md-5 mt-md-4 pb-5">
  
                <h2 class="fw-bold mb-3 text-uppercase tw-text-4xl">Login</h2>
                <p class="text-white-50 mb-4">Please enter your login and password!</p>
  
                <div data-mdb-input-init class="form-outline form-white mb-4">
                  <input type="email" id="typeEmailX" class="form-control form-control-lg" />
                  <label class="form-label mt-2" for="typeEmailX">Email</label>
                </div>
  
                <div data-mdb-input-init class="form-outline form-white mb-4">
                  <input type="password" id="typePasswordX" class="form-control form-control-lg" />
                  <label class="form-label mt-2" for="typePasswordX">Password</label>
                </div>
  
                <p class="small mb-4 pb-lg-2"><a class="text-white-50" href="#!">Forgot password?</a></p>
  
                <button data-mdb-button-init data-mdb-ripple-init class="btn btn-outline-light btn-lg px-5 " type="submit">Login</button>
  
              </div>
  
              <div>
                <p class="mb-2">Don't have an account? <a href="/user/register" class="text-white-50 fw-bold">Sign Up</a>
                </p>
              </div>
  
            </div>
          </div>
        </div>
      </div>
    </div>
    <img src={logo} alt="Nexis Logo" width="100px" height="100px" style="position: absolute; bottom: 10px; right: 10px;">
  </section>

  <script>
    import logo from '$lib/assets/Nexis.png';
  </script>