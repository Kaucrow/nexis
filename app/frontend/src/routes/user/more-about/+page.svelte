<script lang='ts'>
    import Checkbox from '$lib/components/Checkbox.svelte';
    import logo from '$lib/assets/Nexis.png';
    import mall from '$lib/assets/mall.jpg';
</script>

<style>

    @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap');
  
    .gradient-custom {
      font-family: 'Poppins', sans-serif;
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
      width: 40%; 
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
      width: 100%;
    }
    .login-card{
      border-top: 8px solid #387478;
    }
    ::placeholder {
      color: #ccc;
      opacity: 1;
    }
  
    .password-container {
    position: relative;
  }
  .toggle-password {
    position: absolute;
    right: 10px;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    cursor: pointer;
  }
  .toggle-password svg {
    width: 24px;
    height: 24px;
  }
  .checkbox-container {
    display: flex;
    align-items: center;
    margin-bottom: 1rem;
  }
  .checkbox-container p {
    margin-left: 1rem;
    flex: 1;
  }
  .custom-width {
    max-width: 850px; /* Ajusta este valor según sea necesario */
  }
  
  .next-button, .skip-button {
    padding: 10px 20px;
    font-size: 16px;
    margin: 0 5px;/* Espacio entre los botones */
  }
  .skip-button {
    padding: 10px 10px;
    width: 20%; /* Hacer el botón más estrecho */
    background-color: #387478;
    margin-left: auto;
  }
  .next-button-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }
  
  </style>
  
  <section class="vh-100 gradient-custom tw-bg-dark-background" style="position: relative;">
    <!-- svelte-ignore a11y-img-redundant-alt -->
    <img src={mall} alt="Mall Image" class="full-height-image">
    <div class="container py-4 h-100">
      <div class="row d-flex justify-content-center align-items-center h-100 ">
        <div class="col-12 col-md-10 col-lg-8 col-xl-7 custom-width login-container">
          <div class="card tw-bg-dark-card text-white login-card" style="border-radius: 25px;">
            <div class="card-body tw-px-10 p-4 pt-2 pb-1 custom-height" style="width: 100%;">
  
              <div class="mb-md-2 mt-md-4 pb-4">
                <h3 class="pt-4 mb-4 tw-text-xl tw-text-dark-primary">Almost there!</h3>
                <h2 class="mb-4 tw-text-2xl tw-text-dark-primary">Tell us a bit more about yourself</h2>
                <p class="mb-5 tw-text-dark-primary">Which stores are you interested in?</p>
                
                <!-- Checkboxes Section -->
                <div class="row tw-pt-2 tw-pb-4 tw-mb-4">
                  <div class="col-6 checkbox-container">
                    <Checkbox id="option1" class="mr-4" />
                    <p class="tw-pl-4 ">Readon: A library offering a variety of books, pens, notebooks, and other stationery items.</p>
                    <svg width="48" height="48" class="ml-2">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M96 0C43 0 0 43 0 96L0 416c0 53 43 96 96 96l288 0 32 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l0-64c17.7 0 32-14.3 32-32l0-320c0-17.7-14.3-32-32-32L384 0 96 0zm0 384l256 0 0 64L96 448c-17.7 0-32-14.3-32-32s14.3-32 32-32zm32-240c0-8.8 7.2-16 16-16l192 0c8.8 0 16 7.2 16 16s-7.2 16-16 16l-192 0c-8.8 0-16-7.2-16-16zm16 48l192 0c8.8 0 16 7.2 16 16s-7.2 16-16 16l-192 0c-8.8 0-16-7.2-16-16s7.2-16 16-16z"/></svg>
                    </svg>
                  </div>
                  <div class="col-6 checkbox-container tw-ml">
                    <Checkbox class="mr-4" id="option2" />
                    <p class="tw-pl-4">Cyberion: A tech store featuring a wide selection of phones, computers, and various gadgets.</p>
                    <svg width="48" height="48" class="ml-2">
                      <use href="/path/to/your/svg2.svg#icon"></use>
                    </svg>
                  </div>
                </div>
                <div class="row tw-pt-2 tw-pb-4 tw-mb-4">
                  <div class="col-6 checkbox-container">
                    <Checkbox id="option3" class="mr-4" />
                    <p class="tw-pl-4">Vesti: A clothing store which caters to all your clothing needs, from shoes to shirts.</p>
                    <svg width="24" height="24" class="ml-2">
                      <use href="/path/to/your/svg3.svg#icon"></use>
                    </svg>
                  </div>
                  <div class="col-6 checkbox-container">
                    <Checkbox id="option4" class="mr-4" />
                    <p class="tw-pl-4">Savoro: A food store. Your go-to place for all things foody.</p>
                    <svg width="24" height="24" class="ml-2">
                      <use href="/path/to/your/svg4.svg#icon"></use>
                    </svg>
                  </div>
                </div>
                <!-- End of Checkboxes Section -->

                <div class="button-container next-button-container" style="position: relative;">
                  <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom fw-bold tw-mt-3 btn-small next-button" type="submit">Next</button>
                  <button data-mdb-button-init data-mdb-ripple-init class="btn btn-custom fw-bold tw-mt-3 btn-small skip-button" type="submit">Skip</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    <img src={logo} alt="Nexis Logo" width="100px" height="100px" style="position: absolute; top: 10px; right: 10px;">
</section>