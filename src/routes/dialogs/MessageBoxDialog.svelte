<!-- MessageBoxDialog.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  // Dialog props
  const { 
    type = "ok", 
    title = "Message", 
    message = "", 
    onResult = (result: string) => {},
    showInput = false,
    inputLabel = "",
    inputPlaceholder = ""
  } = $props();
  
  let inputValue = $state("");
  let isClosing = $state(false);
  
  // Animation timers
  let closeTimer: number | null = null;
  
  onMount(() => {
    console.log("MessageBox dialog mounted:", { type, title });
  });
  
  onDestroy(() => {
    if (closeTimer !== null) {
      clearTimeout(closeTimer);
    }
  });
  
  function handleResult(result: string) {
    // Start closing animation
    isClosing = true;
    
    // Add a small delay for the animation to complete
    closeTimer = setTimeout(() => {
      // Send the result, with input value if applicable
      if (showInput) {
        onResult(`${result}:${inputValue}`);
      } else {
        onResult(result);
      }
    }, 200) as unknown as number;
  }
  
  function handleOk() {
    handleResult("ok");
  }
  
  function handleYes() {
    handleResult("yes");
  }
  
  function handleNo() {
    handleResult("no");
  }
  
  function handleCancel() {
    handleResult("cancel");
  }
  
  function handleKeydown(event: KeyboardEvent) {
    // Handle Escape key
    if (event.key === "Escape") {
      if (type === "yes/no/cancel" || type === "ok/cancel") {
        handleCancel();
      } else {
        handleOk();
      }
    }
    
    // Handle Enter key
    if (event.key === "Enter") {
      if (type === "yes/no/cancel") {
        handleYes();
      } else {
        handleOk();
      }
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="dialog-overlay" class:closing={isClosing}>
  <div class="dialog" class:closing={isClosing}>
    <div class="dialog-header">
      <h3>{title}</h3>
    </div>
    <div class="dialog-content">
      <p>{message}</p>
      {#if showInput}
        <div class="input-container">
          {#if inputLabel}
            <label for="dialog-input">{inputLabel}</label>
          {/if}
          <input 
            id="dialog-input"
            type="text" 
            bind:value={inputValue} 
            placeholder={inputPlaceholder} 
            autofocus
          />
        </div>
      {/if}
    </div>
    <div class="dialog-footer">
      {#if type === "yes/no/cancel"}
        <button onclick={handleYes} class="primary-button">Yes</button>
        <button onclick={handleNo} class="secondary-button">No</button>
        <button onclick={handleCancel} class="cancel-button">Cancel</button>
      {:else if type === "yes/no"}
        <button onclick={handleYes} class="primary-button">Yes</button>
        <button onclick={handleNo} class="secondary-button">No</button>
      {:else if type === "ok/cancel"}
        <button onclick={handleOk} class="primary-button">OK</button>
        <button onclick={handleCancel} class="cancel-button">Cancel</button>
      {:else}
        <button onclick={handleOk} class="primary-button">OK</button>
      {/if}
    </div>
  </div>
</div>

<style>
  .dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.7);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
    animation: fadeIn 0.2s ease-out;
  }
  
  .dialog {
    background-color: var(--bg-secondary);
    border-radius: 8px;
    box-shadow: 0 8px 24px var(--shadow-color);
    min-width: 350px;
    max-width: 95%;
    overflow: hidden;
    animation: slideIn 0.3s ease-out;
    border: 1px solid var(--border-color);
  }
  
  .dialog-header {
    padding: 16px;
    background-color: var(--bg-tertiary);
    border-bottom: 1px solid var(--border-color);
  }
  
  .dialog-header h3 {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 500;
    color: var(--text-primary);
    text-align: center;
  }
  
  .dialog-content {
    padding: 24px 20px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  
  .dialog-content p {
    font-size: 1.1rem;
    margin: 0 0 16px 0;
    color: var(--text-primary);
    text-align: center;
  }
  
  .dialog-footer {
    padding: 16px;
    display: flex;
    justify-content: center;
    gap: 12px;
    border-top: 1px solid var(--border-color);
  }
  
  .primary-button, .secondary-button, .cancel-button {
    border-radius: 6px;
    padding: 10px 20px;
    font-size: var(--default_fontsize);
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 1px 3px var(--shadow-color);
  }
  
  .primary-button {
    background-color: var(--accent-color);
    color: white;
    border: 1px solid var(--accent-color-dark);
  }
  
  .primary-button:hover {
    background-color: var(--accent-color-dark);
    transform: translateY(-1px);
  }
  
  .secondary-button {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }
  
  .secondary-button:hover {
    background-color: var(--bg-quaternary);
    transform: translateY(-1px);
  }
  
  .cancel-button {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-left: 3px solid var(--warning-color);
  }
  
  .cancel-button:hover {
    background-color: var(--accent-color-transparent);
    border-color: var(--warning-color);
    transform: translateY(-1px);
  }
  
  .cancel-button:active, .primary-button:active, .secondary-button:active {
    transform: translateY(0);
  }
  
  /* Input field styling */
  .input-container {
    width: 100%;
    margin-bottom: 16px;
    padding: 0 4px;
  }
  
  label {
    display: block;
    margin-bottom: 8px;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }
  
  input {
    width: 100%;
    padding: 10px;
    border-radius: 4px;
    border: 1px solid var(--border-color);
    background-color: var(--bg-input);
    color: var(--text-primary);
    font-size: 1rem;
    box-sizing: border-box;
  }
  
  input:focus {
    outline: none;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 2px var(--accent-color-transparent);
  }
  
  /* Animation classes */
  .dialog-overlay.closing {
    animation: fadeOut 0.2s ease-in forwards;
  }
  
  .dialog.closing {
    animation: slideOut 0.2s ease-in forwards;
  }
  
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }
  
  @keyframes fadeOut {
    from { opacity: 1; }
    to { opacity: 0; }
  }
  
  @keyframes slideIn {
    from {
      transform: translateY(-20px);
      opacity: 0;
    }
    to {
      transform: translateY(0);
      opacity: 1;
    }
  }
  
  @keyframes slideOut {
    from {
      transform: translateY(0);
      opacity: 1;
    }
    to {
      transform: translateY(-20px);
      opacity: 0;
    }
  }
</style>