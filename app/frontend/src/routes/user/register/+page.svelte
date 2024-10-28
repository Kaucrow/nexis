<section class="vh-100 gradient-custom " style="background-color: #090d10">
    <div class="container py-5 h-100">
      <div class="row justify-content-center align-items-center h-100">
        <div class="col-12 col-lg-9 col-xl-7">
          <div class="card shadow-2-strong card-registration tw-bg-primary-dark text-white" style="border-radius: 15px;">
            <div class="card-body p-4 p-md-5">
              <h3 class="mb-4 pb-2 pb-md-0 mb-md-5 tw-text-4xl text-center">Registration Form</h3>
              <form>
  
                <div class="row">
                  <div class="col-md-6 mb-4">
  
                    <div data-mdb-input-init class="form-outline tw-text-center">
                      <input type="text" id="firstName" class="form-control form-control-lg " />
                      <label class="form-label" for="firstName">First Name</label>
                    </div>
  
                  </div>
                  <div class="col-md-6 mb-4">
  
                    <div data-mdb-input-init class="form-outline tw-text-center">
                      <input type="text" id="lastName" class="form-control form-control-lg" />
                      <label class="form-label" for="lastName">Last Name</label>
                    </div>
  
                  </div>
                </div>
  
                <div class="row">
                  <div class="col-md-6 mb-4 d-flex align-items-center">
  
                    <div data-mdb-input-init class="form-outline datepicker w-100 tw-text-center">
                      <input type="text" class="form-control form-control-lg" id="birthdayDate" />
                      <label for="birthdayDate" class="form-label">Birthday</label>
                    </div>
  
                  </div>
                  <div class="col-md-6 mb-4">
  
                    <h6 class="mb-2 pb-1">Gender: </h6>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="femaleGender"
                        value="option1" checked />
                      <label class="form-check-label" for="femaleGender">Female</label>
                    </div>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="maleGender"
                        value="option2" />
                      <label class="form-check-label" for="maleGender">Male</label>
                    </div>
  
                    <div class="form-check form-check-inline">
                      <input class="form-check-input" type="radio" name="inlineRadioOptions" id="otherGender"
                        value="option3" />
                      <label class="form-check-label" for="otherGender">Other</label>
                    </div>
  
                  </div>
                </div>
  
                <div class="row">
                  <div class="col-md-6 mb-4 pb-2">
  
                    <div data-mdb-input-init class="form-outline tw-text-center">
                      <input type="email" id="emailAddress" class="form-control form-control-lg" />
                      <label class="form-label" for="emailAddress">Email</label>
                    </div>
  
                  </div>
                  <div class="col-md-6 mb-4 pb-2">
  
                    <div data-mdb-input-init class="form-outline tw-text-center">
                      <input type="tel" id="phoneNumber" class="form-control form-control-lg" />
                      <label class="form-label" for="phoneNumber">Phone Number</label>
                    </div>
  
                  </div>
                </div>

                
                <div class="mt-2 d-flex justify-content-center align-items-center">
                    <button data-mdb-button-init data-mdb-ripple-init class="btn btn-outline-light btn-lg px-5" type="submit">Register</button>
                </div>

                <div class="mt-3 pt-2 d-flex justify-content-center align-items-center">
                    <p class="mt-2 mb-0">Already have an account? <a href="/user/login" class="text-white-50 fw-bold">Sign In</a></p>
                </div>
  
              </form>
              
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