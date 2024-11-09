<svelte:head>
    <title>Checkout</title>
</svelte:head>

<script lang="ts">
    import logo from '$lib/assets/Nexis.png';
    import graphicShirt from '$lib/assets/graphic-shirt.jpg';
    import checkeredShirt from '$lib/assets/checkered-shirt.jpg';
    import jeans from '$lib/assets/jeans.jpg';
    import { CreditCard, Wallet, Landmark } from 'lucide-svelte';

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

    $: netAmount = products.reduce((sum, product) => sum + (product.price * product.quantity), 0);
    $: discount = 20.00;
    $: total = netAmount - discount;

    let selectedPaymentMethod = '';
    let transferReference = ''

    const paymendMethods = [
        {
            id: 'credit-card',
            name: "Credit Card",
            icon: CreditCard
        },
        {
            id: 'paypal',
            name: "PayPal",
            icon: Wallet
        },
        {
            id: 'bank-transfer',
            name: "Bank Transfer",
            icon: Landmark
        }
    ]
    const bankInfo = {
      accountID: "J-1234567890",
      accountNumber: "1234567890",
      bankName: "Bank of America",
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

</style>

<div class="tw-min-h-screen tw-p-14 scrollable-content">
    <div class="tw-mx-auto tw-max-w-[1400px]">
      <div class="tw-mb-14">
        <h1 class="tw-text-[#387478] tw-text-3xl tw-font-medium">Checkout</h1>
        <a href="/client/cart" class="tw-text-sm tw-text-[#E2F1E7] hover:tw-text-[#387478] tw-no-underline">
          ← Back to Cart
        </a>
      </div>
  
      <div class="tw-grid tw-gap-24 lg:tw-grid-cols-[1fr,390px] lg:tw-items-start">
        <div class="tw-rounded-2xl tw-border-4 tw-border-[#1A252E] tw-p-4 tw-bg-[#0d1317]">
          <div class="scrollable-content tw-space-y-3">
            {#each products as product, i}
              <div class="product-card tw-flex tw-items-center tw-justify-between tw-rounded-lg tw-p-3 tw-relative">
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
              </div>
            {/each}
          </div>
        </div>
  
        <div class="tw-rounded-2xl tw-bg-[#1A252E] tw-p-6 tw-border-t-[6px] tw-border-[#387478] lg:tw-sticky lg:tw-top-6">
          <h2 class="tw-text-2xl tw-font-medium tw-text-[#E2F1E7] tw-mb-6">Order Summary</h2>
          
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
                
        <div class="tw-mt-3 tw-space-y-4">
            <h3 class="tw-text-base tw-text-[#E2F1E7]">Payment Method</h3>
            <div class="tw-grid tw-grid-cols-3 tw-gap-3">
              {#each paymendMethods as method}
                <button
                  class="w-flex tw-flex-col tw-items-center tw-justify-center tw-p-3 tw-rounded-lg tw-bg-[#21313B] tw-transition-colors hover:tw-bg-[#2A3744] focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-[#387478] {selectedPaymentMethod === method.id ? 'tw-ring-2 tw-ring-[#387478]' : ''}"
                  on:click={() => selectedPaymentMethod = method.id}
                >
                  <svelte:component this={method.icon} class="tw-w-5 tw-h-5 tw-text-[#629584]" />
                  <span class="tw-text-xs tw-text-[#E2F1E7]">{method.name}</span>
                </button>
              {/each}
            </div>
            
            {#if selectedPaymentMethod === 'bank-transfer'}
              <div class="tw-mt-4 tw-p-4 tw-bg-[#21313B] tw-rounded-lg">
                <h4 class="tw-text-sm tw-font-medium tw-text-[#E2F1E7] tw-mb-2">Bank Account Information</h4>
                <p class="tw-text-xs tw-text-[#8B8B8B]">Account ID: {bankInfo.accountID}</p>
                <p class="tw-text-xs tw-text-[#8B8B8B]">Account Number: {bankInfo.accountNumber}</p>
                <p class="tw-text-xs tw-text-[#8B8B8B]">Bank Name: {bankInfo.bankName}</p>
            
                <div class="tw-mt-4">
                  <label for="transfer-reference" class="tw-text-sm tw-text-[#E2F1E7] tw-block tw-mb-1">Transfer Reference</label>
                  <input
                    type="text"
                    id="transferReference"
                    bind:value={transferReference}
                    placeholder="Enter transfer reference"
                    class="tw-w-full tw-bg-[#0D1317] tw-text-[#E2F1E7] tw-border tw-border-[#387478] tw-rounded-md tw-px-3 tw-py-2 tw-text-sm focus:tw-outline-none focus:tw-ring-2 focus:tw-ring-[#387478]"
                  />
                </div>
              </div>
            {/if}

            <button 
              class="tw-w-full tw-bg-[#387478] tw-text-[#162027] tw-rounded-3xl tw-py-3.5 tw-text-base tw-font-medium hover:tw-bg-[#629584] tw-transition-colors {!selectedPaymentMethod ? 'tw-opacity-50 tw-cursor-not-allowed' : ''}"
              disabled={!selectedPaymentMethod}
            >
            <a href="/client/paid">
              Pay 
            </a>
            </button>
          </div>
        
         

      </div>  
    </div>
    </div>  
    
    <img src={logo} alt="Nexis Logo" class="tw-fixed tw-top-14 tw-right-14 tw-w-12 tw-h-12">
  </div>

