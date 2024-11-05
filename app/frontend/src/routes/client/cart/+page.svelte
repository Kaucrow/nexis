<svelte:head>
  <title>Shopping Cart</title>
</svelte:head>

<script>
  import logo from '$lib/assets/Nexis.png';
  import graphicShirt from '$lib/assets/graphic-shirt.jpg';
  import checkeredShirt from '$lib/assets/checkered-shirt.jpg';
  import jeans from '$lib/assets/jeans.jpg';

  import { Trash2 } from 'lucide-svelte';

  let products = [
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
    {
      id: 3,
      name: "Skinny Fit Jeans",
      size: "Medium",
      store: "Vesti",
      price: 249.99,
      image: jeans,
      quantity: 1
    },
    {
      id: 4,
      name: "Skinny Fit Jeans",
      size: "Medium",
      store: "Vesti",
      price: 200.9,
      image: jeans,
      quantity: 1
    }
    
  ];

  let promoCode = "";

  function updateQuantity(index = 0, change = 0) {
    products[index].quantity = Math.max(1, products[index].quantity + change);
    products = [...products];
  }

  $: netAmount = products.reduce((sum, product) => sum + (product.price * product.quantity), 0);
  $: discount = 20.00;
  $: total = netAmount - discount;

  function deleteProduct(index = 0) {
    products = products.filter((_, i) => i !== index);
  }
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


  .scrollable-content {
    max-height: 69vh;
    overflow-y: auto;
    padding-right: 15px;
    margin-right: -1px;
  }
  .scrollable-content::-webkit-scrollbar {
    width: 3px;
  }

  .scrollable-content::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
  }

  .scrollable-content::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.1);
    border-radius: px;
  }

  .product-card {
    border: 1px transparent rgba(255, 255, 255, 0.1);
    position: absolute;
  }
  .product-card:not(:last-child)::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: calc(100% - 2rem);
    height: 1px;
    background: #21313B;
  }

  .quantity-control {
    background: #387478;
    border-radius: 14px;
    padding: 4px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-width: 90px;
  }

  .quantity-btn {
    background: #387478;
    border: none;
    color: black;
    width: 24px;
    height: 24px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    padding: 0;
    transition: opacity 0.2s;
    font-size: 16px;
    line-height: 1;

  }

  .quantity-btn:hover {
    opacity: 0.8;
  }


  .delete-btn {
    color: #FF4747;
    background: transparent;
    border: none;
    padding: 0px;
    cursor:pointer;
    transition: opacity 0.2;
    position: absolute;
    top: 1px;
    right: 1px;
  }

  .delete-btn:hover {
    opacity: 0.8;
  }
</style>

<div class="tw-min-h-screen tw-p-14 ">
  <div class="tw-mx-auto tw-max-w-[1400px]">
    <div class="tw-mb-14">
      <h1 class="tw-text-[#387478] tw-text-3xl tw-font-medium">My Cart</h1>
      <a href="#" class="tw-text-sm tw-text-[#E2F1E7] hover:tw-text-[#387478] tw-no-underline">
        ← Back to Shop
      </a>
    </div>

    <div class="tw-grid tw-gap-24 lg:tw-grid-cols-[1fr,390px] lg:tw-items-start">
      <div class="tw-rounded-2xl tw-border-4 tw-border-[#1A252E] tw-p-4 tw-bg-[#0d1317]">
        <div class="scrollable-content tw-space-y-3">
          {#each products as product, i}
            <div class="product-card tw-flex tw-items-center tw-justify-between tw-rounded-lg tw-p-3 tw-relative">
              <button class="delete-btn" on:click={() => deleteProduct(i)} aria-label="Delete product"><Trash2 size="19" /></button>
              <div class="tw-flex tw-items-center tw-gap-3">
                <img
                  src={product.image}
                  alt={product.name}
                  class="tw-h-16 tw-w-16 tw-rounded-md tw-bg-white tw-object-cover"
                />
                <div>
                  <h3 class="tw-text-sm tw-font-medium tw-text-[#E2F1E7] tw-mb-1">{product.name}</h3>
                  <p class="tw-text-xs tw-text-[#8B8B8B] tw-mb-0.5">Size: {product.size}</p>
                  <p class="tw-text-xs tw-text-[#8B8B8B] tw-mb-1">Store: {product.store}</p>
                  <p class="tw-text-sm tw-font-medium tw-text-white">${product.price.toFixed(1)}</p>
                </div>
              </div>
              <div class="quantity-control">
                <button class="quantity-btn" on:click={() => updateQuantity(i, -1)} aria-label="Decrease quantity">-</button>
                <span class="quantity-btn">{product.quantity}</span>
                <button class="quantity-btn" on:click={() => updateQuantity(i, 1)} aria-label="Increase quantity">+</button>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="tw-rounded-2xl tw-bg-[#1A252E] tw-p-6 tw-border-t-[6px] tw-border-[#387478] lg:tw-sticky lg:tw-top-6">
        <h2 class="tw-text-2xl tw-font-medium tw-text-[#E2F1E7] tw-mb-5">Order Summary</h2>
        
        <div class="tw-space-y-3">
          <div class="tw-flex tw-justify-between tw-text-sm tw-items-center">
            <span class="tw-text-[#629584]">Net amount:</span>
            <span class="tw-text-[#629584]">${netAmount.toFixed(1)}</span>
          </div>
          <div class="tw-flex tw-justify-between tw-text-sm tw-items-center">
            <span class="tw-text-[#FFFFFF]">Discount:</span>
            <span class="tw-text-[#FFFFFF]">${discount.toFixed(1)}</span>
          </div>
          
          <div class="tw-relative tw-py-1">
            <div class="tw-absolute tw-left-0 tw-right-0 tw-h-[1px] tw-bg-[#21313B]"></div>
          </div>

          <div class="tw-flex tw-justify-between tw-text-sm tw-items-center">
            <span class="tw-text-[#FFFFFF] tw-font-medium">Total:</span>
            <span class="tw-text-[#FFFFFF] tw-font-medium">${total.toFixed(1)}</span>
          </div>
        </div>
        
        <div class="tw-mt-8 tw-space-y-4">
          <div class="tw-flex tw-gap-3">
            <div class="tw-relative tw-flex-1">
              <div class="tw-absolute tw-left-3 tw-top-1/2 tw--translate-y-1/2">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#629584" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/>
                  <line x1="7" y1="7" x2="7.01" y2="7"/>
                </svg>
              </div>
              <input
                type="text"
                placeholder="Add promo code"
                bind:value={promoCode}
                class="tw-w-full tw-bg-[#21313B] tw-border-none tw-rounded-3xl tw-py-3 tw-pl-10 tw-pr-4 tw-text-[#E2F1E7] tw-text-sm placeholder:tw-text-[#629584] focus:tw-outline-none focus:tw-ring-1 focus:tw-ring-[#387478]"
              />
            </div>
            <button class="tw-bg-[#387478] tw-text-[#162027] tw-px-6 tw-rounded-3xl tw-text-sm tw-font-medium hover:tw-bg-[#629584] tw-transition-colors">
              Apply
            </button>
          </div>
          <button class="tw-w-full tw-bg-[#387478] tw-text-[#162027] tw-rounded-3xl tw-py-3.5 tw-text-base tw-font-medium hover:tw-bg-[#629584] tw-transition-colors">
            <a href="/client/checkout">  Checkout  </a>
          </button>
        </div>
      </div>
    </div>  
  </div>
  
  
  <img src={logo} alt="Nexis Logo" class="tw-absolute tw-top-14 tw-right-14 tw-w-12 tw-h-12">
</div>