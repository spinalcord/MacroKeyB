<!-- DetectKeyboardDialog.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";

  // Use $props in Svelte 5
  const { onDetectDone = (result: string) => {} } = $props();
  
  let isWaiting = $state(true);
  let message = $state("Press any key you want to use as a Macro device...");
  let detectionCompleted = $state(false);
  let detectionTimeout: number | null = null;
  
  // Animation for waiting
  let dots = $state("");
  let dotsInterval: number | null = null;

  onMount(() => {
    console.log("Dialog was mounted");
    // Start the detection
    startDetection();
    
    // Set a local timeout as an additional safeguard
    detectionTimeout = setTimeout(() => {
      if (!detectionCompleted) {
        cancelDetection("Timeout in client");
      }
    }, 31000); // Slightly longer than the server timeout
    
    // Animated dots for better feedback
    dotsInterval = setInterval(() => {
      dots = dots.length >= 3 ? "" : dots + ".";
    }, 500) as unknown as number;
  });
  
  onDestroy(() => {
    console.log("Dialog is being destroyed, cleaning up");
    
    // Clear the timeout if it exists
    if (detectionTimeout !== null) {
      clearTimeout(detectionTimeout);
    }
    
    // Clear the dots animation
    if (dotsInterval !== null) {
      clearInterval(dotsInterval);
    }
    
    // Do nothing if detection is already completed
    if (detectionCompleted) return;
    
    // If we cancel manually without getting a result
    if (!detectionCompleted && isWaiting) {
      cancelDetection("Cancelled by closing");
    }
  });
  
  function handleCompletion(result: string) {
    // Prevent multiple calls
    if (detectionCompleted) return;
    
    detectionCompleted = true;
    isWaiting = false;
    onDetectDone(result);
  }
  
  async function startDetection() {
    console.log("Starting new detection");
    isWaiting = true;
    message = "Press any key you want to use as a Macro device";
    detectionCompleted = false;
    
    try {
      // Start the detection
      const result = await invoke("wait_for_keypress");
      console.log("Detection successful:", result);
      handleCompletion(result as string);
    } catch (e) {
      console.error("Error during detection:", e);
      handleCompletion(`Error: ${e}`);
    }
  }
  
  async function cancelDetection(reason: string = "User cancelled") {
    console.log(`Cancelling detection: ${reason}`);
    message = "Cancelling...";
    
    try {
      // IMPORTANT: Call the backend function to stop the detection
      await invoke("cancel_keypress_detection");
      console.log("Backend detection successfully cancelled");
    } catch (e) {
      console.error("Error cancelling detection:", e);
    } finally {
      // Close the dialog even on error
      handleCompletion(reason);
    }
  }
  
  function cancel() {
    cancelDetection("User cancelled");
  }
</script>

<div class="dialog-overlay">
  <div class="dialog">
    <div class="dialog-header">
      <h3>Detect Input Device</h3>
    </div>
    <div class="dialog-content">
      <div class="message-container">
        <p>{message}{isWaiting ? dots : ""}</p>
        {#if isWaiting}
          <div class="pulse-ring"></div>
        {/if}
      </div>
    </div>
    <div class="dialog-footer">
      <button onclick={cancel} class="cancel-button" disabled={!isWaiting || detectionCompleted}>
        Cancel
      </button>
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
    padding: 24px 16px;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  
  .message-container {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 80px;
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
    border-top: 1px solid var(--border-color);
  }
  
  .cancel-button {
    background-color: var(--bg-tertiary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-left: 3px solid var(--warning-color);
    border-radius: 6px;
    padding: 10px 20px;
  font-size: var(--default_fontsize);

    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 1px 3px var(--shadow-color);
  }
  
  .cancel-button:hover:not(:disabled) {
    background-color: var(--accent-color-transparent);
    border-color: var(--warning-color);
    transform: translateY(-1px);
  }
  
  .cancel-button:active:not(:disabled) {
    background-color: var(--bg-tertiary);
    transform: translateY(0);
  }
  
  .cancel-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  /* Pulsating circle animation */
  .pulse-ring {
    position: relative;
    width: 30px;
    height: 30px;
    margin: 10px auto;
  }
  
  .pulse-ring:before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border-radius: 50%;
    background-color: var(--accent-color);
    opacity: 0.4;
    animation: pulse 2s infinite;
  }
  
  .pulse-ring:after {
    content: '';
    position: absolute;
    top: 9px;
    left: 9px;
    right: 9px;
    bottom: 9px;
    border-radius: 50%;
    background-color: var(--accent-color);
  }
  
  @keyframes pulse {
    0% {
      transform: scale(0.5);
      opacity: 0.8;
    }
    70% {
      transform: scale(1.5);
      opacity: 0;
    }
    100% {
      transform: scale(0.5);
      opacity: 0;
    }
  }
  
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
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
</style>