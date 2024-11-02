<svelte:head>
  <title>Shopping Cart</title>
</svelte:head>

<script>
  import logo from '$lib/assets/Nexis.png';
  import graphicShirt from '$lib/assets/graphic-shirt.jpg';
  import checkeredShirt from '$lib/assets/checkered-shirt.jpg';
  import jeans from '$lib/assets/jeans.jpg';

  // Static Data for the products in the cart
  const products = [
    {
      id: 0,
      name: "Gradient Graphic T-shirt",
      size: "Large",
      store: "Vesti",
      price: 145.0,
      image: graphicShirt,
      quantity: 1
    },
    {
      id: 1,
      name: "Checkered Shirt",
      size: "Medium",
      store: "Vesti",
      price: 200.0,
      image: checkeredShirt,
      quantity: 1
    },
    {
      id: 2,
      name: "Skinny Fit Jeans",
      size: "Medium",
      store: "Vesti",
      price: 249.99,
      image: jeans,
      quantity: 1
    },
  ];

  let promoCode = "";
</script>

<style>
  @import url('https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600&display=swap');

  :global(body) {
    font-family: 'Poppins', sans-serif;
    background-color: #131A20;
  }
  

  /* Custom Scrollbar Styles */
  .scrollable-content::-webkit-scrollbar {
    width: 6px;
  }

  .scrollable-content::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 3px;
  }

  .scrollable-content::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
    border-radius: 3px;
  }

  .scrollable-content::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }

  :global(.tw-shadow-custom) {
    box-shadow: 0px 4px 64px rgba(0, 0, 0, 0.05);
  }
</style>

<div class="tw-min-h-screen tw-bg-primary-background tw-p-6">
  <div class="tw-mx-auto tw-max-w-6xl">
    <div class="tw-mb-8 tw-flex tw-items-center tw-justify-between">
      <div>
        <h1 class="tw-text-2xl tw-font-semibold tw-text-primary-light">My Cart</h1>
        <a href="#" class="tw-text-sm tw-text-primary-secondary hover:tw-text-accent">
          ← Back to Shop
        </a>
      </div>
      <img src={logo} alt="Nexis Logo" class="tw-w-10 tw-h-10">
    </div>

    <div class="tw-grid tw-gap-6 lg:tw-grid-cols-[1fr,400px]">
      <div class="tw-rounded-xl tw-bg-[#1A252E] tw-p-6 tw-shadow-lg tw-shadow-black/20">
        <div class="scrollable-content tw-max-h-[600px] tw-space-y-4 tw-overflow-y-auto tw-pr-2">
          {#each products as product (product.id)}
            <div
              class="tw-flex tw-items-center tw-justify-between tw-rounded-lg tw-bg-[#21313B] tw-p-4 tw-shadow-md"
            >
              <div class="tw-flex tw-items-center tw-gap-4">
                <img
                  src={product.image}
                  alt={product.name}
                  class="tw-h-20 tw-w-20 tw-rounded-md tw-bg-white tw-object-cover"
                />
                <div>
                  <div class="tw-flex tw-items-center tw-gap-2">
                    <h3 class="tw-font-medium tw-text-primary-light">{product.name}</h3>
                  </div>
                  <p class="tw-text-sm tw-text-[#8B8B8B]">Size: {product.size}</p>
                  <p class="tw-text-sm tw-text-[#8B8B8B]">Store: {product.store}</p>
                  <p class="tw-mt-1 tw-font-medium tw-text-primary-secondary">${product.price.toFixed(2)}</p>
                </div>
              </div>
              <div class="tw-flex tw-items-center tw-gap-3 tw-rounded-lg tw-bg-[#1A252E] tw-px-3 tw-py-2">
                <span class="tw-w-8 tw-text-center tw-text-primary-light">{product.quantity}</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="tw-rounded-[25px] tw-bg-[#1a252e] tw-p-6 tw-shadow-custom tw-border-t-[10px] tw-border-[#387478] tw-h-[461px] tw-overflow-y-auto">
        <h2 class="tw-mb-8 tw-text-2xl tw-font-semibold tw-text-primary-light">Order Summary</h2>
        <div class="tw-space-y-6">
          <div class="tw-flex tw-justify-between tw-text-[#E2F1E7]">
            <span class="tw-text-lg">Net amount:</span>
            <span class="tw-text-lg">$594.99</span>
          </div>
          <div class="tw-flex tw-justify-between tw-text-[#E2F1E7]">
            <span class="tw-text-lg">Discount:</span>
            <span class="tw-text-lg">$20.00</span>
          </div>
          <div class="tw-flex tw-justify-between tw-text-xl tw-font-medium tw-text-primary-light tw-mt-2">
            <span>Total:</span>
            <span>$574.99</span>
          </div>
          <div class="tw-relative tw-mt-8">
            <div class="tw-absolute tw-left-3 tw-top-1/2 tw--translate-y-1/2 tw-text-primary-secondary">
              🏷️
            </div>
            <input
              type="text"
              placeholder="Add promo code"
              bind:value={promoCode}
              class="tw-w-full tw-rounded-lg tw-bg-[#1A252E] tw-py-3 tw-pl-10 tw-pr-24 tw-text-sm tw-text-primary-light tw-placeholder-[#629584] tw-shadow-inner"
            />
            <button class="tw-absolute tw-right-2 tw-top-1/2 tw--translate-y-1/2 tw-rounded-md tw-bg-[#387478] tw-px-4 tw-py-1.5 tw-text-sm tw-font-medium tw-text-primary-light hover:tw-bg-[#629584] tw-transition-colors">
              Apply
            </button>
          </div>
          <button class="tw-mt-6 tw-w-full tw-rounded-lg tw-bg-[#387478] tw-py-4 tw-text-lg tw-font-medium tw-text-primary-dark hover:tw-bg-[#629584] tw-transition-colors tw-shadow-lg">
            Checkout
          </button>
        </div>
      </div>
    </div>
  </div>
</div>