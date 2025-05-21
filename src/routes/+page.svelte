<script lang="ts">
    import ListBox from "./components/ListBox.svelte";
    import type { ListBoxItemData } from "./components/ListBoxItemData";
    import { invoke } from "@tauri-apps/api/core";
    import CodeMirror from "svelte-codemirror-editor";
    import { onMount, onDestroy } from "svelte";
    // New imports for theme customization
    import { cmExtensions } from "./codeMirrorModification";
    import DetectKeyboardDialog from "./dialogs/DetectKeyboardDialog.svelte";
    import { eventLuaError } from './stores/eventStore';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    import { Menu } from '@tauri-apps/api/menu';
    import { TrayIcon } from '@tauri-apps/api/tray';
  import { dialogState, showDialog, showAlert, showConfirm, showPrompt } from "./dialogs/dialogManager";
    import MessageBoxDialog from "./dialogs/MessageBoxDialog.svelte";
    import GitHubSvg from './assets/github.svg';


    async function initSystemTray() {
const menu = await Menu.new({
  items: [
    {
      id: 'show',
      text: 'Show',
      action: (id) => {
        appWindow.show();
      },
    },
    {
      id: 'quit',
      text: 'Quit',
      action: (id) => {
        appWindow.destroy();
      },
    },
  ],
});

const options = {
  menu,
  menuOnLeftClick: true,
};

const tray = await TrayIcon.new(options);

    }



 const appWindow = getCurrentWindow();
    appWindow.onCloseRequested((event) => {

      appWindow.hide();
      event.preventDefault();
    });


    let parsedError: {
        status: string;
        itemId: string;
        itemName: string;
        error: string;
        timestamp: string;
    } | null = $state(null);
    
    
     
    // States
    let items: Array<ListBoxItemData> = $state([]);
    let selectedItemId: string | null = $state(null);
    let value = $state("");
    let lastSavedState = $state({});
    let hasUnsavedChanges = $state(false);
    let saveInterval: number;
    
    // States for assignment mode
    let assignModeActive = $state(false);
    let itemWaitingForKey = $state<string | null>(null);
    let statusMessage = $state("");
    let showStatusPopup = $state(false);
    let statusTimeout: number | null = null;
    
    // Dialog state
    let showDetectDialog = $state(false);
    let detectedDevice = "";

    // Helper function to display temporary status messages
    function showTempStatusMessage(message: string, duration: number = 3000) {
        statusMessage = message;
        showStatusPopup = true;
        
        // Clear existing timeout if present
        if (statusTimeout !== null) {
            clearTimeout(statusTimeout);
        }
        
        // Set new timeout
        statusTimeout = setTimeout(() => {
            if (statusMessage === message) {
                showStatusPopup = false;
                setTimeout(() => {
                    statusMessage = "";
                }, 300); // Reset text after hiding
            }
        }, duration) as unknown as number;
    }


    
    // Helper function to update the current item in the list
    function updateItemInList(id: string, updates: Partial<ListBoxItemData>) {
        items = items.map(item => {
            if (item.id === id) {
                return { ...item, ...updates };
            }
            return item;
        });
    }

    // Helper function for error handling
    function handleError(error: any, context: string, duration: number = 5000) {
        console.error(`Error ${context}:`, error);
        const errorMsg = `Error ${context}: ${error}`;
        showTempStatusMessage(errorMsg, duration);
    }


    
    onMount(async () => {
        initSystemTray();
        await initializeData();
        saveInterval = setInterval(autoSaveItems, 30000);
        invoke("load_emited_keyboard");
        checkAssignModeStatus();
    });

    onDestroy(() => {
        clearInterval(saveInterval);
        if (statusTimeout !== null) {
            clearTimeout(statusTimeout);
        }

        

        if (assignModeActive) {
            cancelAssignMode();
        }
        
    });

    async function checkAssignModeStatus() {
        try {
            const [isActive, itemId] = await invoke<[boolean, string | null]>("get_assign_mode_status");
            
            assignModeActive = isActive;
            
            if (itemId && itemId.startsWith("ERROR:")) {
                showTempStatusMessage(itemId.substring(6), 5000);
                itemWaitingForKey = null;
            } else {
                itemWaitingForKey = itemId;
                
                if (isActive) {
                    statusMessage = "Press a key to assign it...";
                    showStatusPopup = true;
                } else if (statusMessage.includes("Press a key")) {
                    showTempStatusMessage("Key assigned successfully.");
                    await initializeData();
                }
            }
        } catch (error) {
            handleError(error, "while fetching assignment mode status");
        }
        
        if (assignModeActive) {
            setTimeout(checkAssignModeStatus, 500);
        }
    }

    function startDetect() {
        console.log("Starting new detection");
        showDetectDialog = true;
    }

    function handleDetectDone(result: string): void {
        console.log("Detection finished with result:", result);
        showDetectDialog = false;
        
        if (result && !result.includes("Cancelled") && !result.includes("Error")) {
            detectedDevice = result;
            showTempStatusMessage("Detected Device: " + detectedDevice, 5000);
        } else {
            console.log("Detection was cancelled or failed");
            if (result.includes("Error")) {
                showTempStatusMessage(result, 5000);
            }
        }
    }

    async function initializeData() {
        try {
            const some_list: [string, string, string, string, boolean][] = await invoke("get_list");
            items = []; 
            
            some_list.forEach(element => {
                items = [...items, {
                    displayText: element[0],
                    assignedKey: element[1],
                    id: element[2],
                    content: element[3],
                    isSelected: element[4],
                }];
            });
            
            const selectedItem = items.find(item => item.isSelected);
            
            if (selectedItem) {
                selectedItemId = selectedItem.id;
                value = selectedItem.content;
            }
            
            updateLastSavedState();
        } catch (error) {
            handleError(error, "while loading data");
        }
    }

    function updateLastSavedState() {
        lastSavedState = JSON.stringify(items);
        hasUnsavedChanges = false;
    }

    async function autoSaveItems() {
        if (hasUnsavedChanges) {
            await saveItems();
        }
    }

    async function saveItems() {
        try {
            const itemsForBackend = items.map(item => [
                item.displayText,
                item.assignedKey,
                item.id,
                item.content,
                item.isSelected
            ]);
            
            await invoke("save_items", { items: itemsForBackend });
            
            updateLastSavedState();
            // showTempStatusMessage("Items saved successfully");
        } catch (error) {
            handleError(error, "while saving");
        }
    }

    async function addItem() {
        try {
            let newItem: [string, string, string, string, boolean] = await invoke("add_item");
            items = [...items, {
                displayText: newItem[0],
                assignedKey: newItem[1],
                id: newItem[2],
                content: newItem[3],
                isSelected: newItem[4],
            }];
            hasUnsavedChanges = true;
            showTempStatusMessage("New item added");
        } catch (error) {
            handleError(error, "while adding");
        }
    }

async function deleteItem() {
    const selectedItem = items.find(item => item.isSelected);
    if (selectedItem) {
        // Show confirmation dialog before deleting
        showConfirm(
            `Are you sure you want to delete "${selectedItem.displayText}"?`,
            "Confirm Deletion",
            async (confirmed) => {
                if (confirmed) {
                    try {
                        await invoke("delete_item", { id: selectedItem.id });
                        items = items.filter(item => !item.isSelected);
                        value = "";
                        selectedItemId = null;
                        hasUnsavedChanges = true;
                        showTempStatusMessage("Item deleted");
                    } catch (error) {
                        handleError(error, "while deleting");
                    }
                } else {
                    // User cancelled deletion
                    showTempStatusMessage("Deletion cancelled");
                }
            }
        );
    } else {
        showTempStatusMessage("Please select an item first");
    }
}

async function renameItem() {
    const selectedItem = items.find(item => item.isSelected);
    if (selectedItem) {
        showPrompt(
            "Enter new name for this item:",
            "Rename Item",
            async (newName) => {
                if (newName && newName.trim() !== "") {
                    try {
                        await invoke("rename_item", { id: selectedItem.id, newName });
                        updateItemInList(selectedItem.id, { displayText: newName });
                        hasUnsavedChanges = true;
                        showTempStatusMessage("Item renamed to: " + newName);
                    } catch (error) {
                        handleError(error, "while renaming");
                    }
                }
            },
            {
                label: "New name",
                placeholder: selectedItem.displayText
            }
        );
    } else {
        showTempStatusMessage("Please select an item first");
    }
}

    async function updateItemContent(event: CustomEvent) {
        const newValue = event.detail;
        
        if (selectedItemId) {
            updateItemInList(selectedItemId, { content: newValue });
            
            try {
                await invoke("update_item_content", { 
                    id: selectedItemId, 
                    content: newValue 
                });
            } catch (error) {
                handleError(error, "while updating");
            }
            
            hasUnsavedChanges = true;
        }
        
        value = newValue;
    }

    async function itemClicked(targetItem: ListBoxItemData) {
        if (selectedItemId) {
            updateItemInList(selectedItemId, { content: value });
            
            try {
                await invoke("update_item_content", { 
                    id: selectedItemId, 
                    content: value 
                });
            } catch (error) {
                handleError(error, "while switching");
            }
        }

        items = items.map(item => ({
            ...item,
            isSelected: item.id === targetItem.id
        }));

        selectedItemId = targetItem.id;
        value = targetItem.content;
        
        try {
            await invoke("select_item", { id: targetItem.id });
        } catch (error) {
            handleError(error, "while selecting");
        }
    }
    
    async function startAssignMode() {
        if (!selectedItemId) {
            showTempStatusMessage("Please select an item first.");
            return;
        }
        
        try {
            await invoke("start_assign_mode", { id: selectedItemId });
            assignModeActive = true;
            itemWaitingForKey = selectedItemId;
            statusMessage = "Press a key to assign it...";
            showStatusPopup = true;
            checkAssignModeStatus();
        } catch (error) {
            handleError(error, "while starting assignment mode");
        }
    }
    
    async function cancelAssignMode() {
        try {
            await invoke("cancel_assign_mode");
            assignModeActive = false;
            itemWaitingForKey = null;
            showTempStatusMessage("Assignment mode cancelled.");
        } catch (error) {
            handleError(error, "while cancelling assignment mode");
        }
    }

    




</script> 

<main>

    <div class="app-container">
        <div class="sidebar">
            <div class="action-buttons-container">
    <button onclick={addItem} class="action-button primary">New</button>
    <button onclick={saveItems} class="action-button primary" disabled={!hasUnsavedChanges}>Speichern</button>
    <button onclick={renameItem} class="action-button delete" disabled={!selectedItemId}>Rename</button>
    <button onclick={deleteItem} class="action-button delete" disabled={!selectedItemId}>Delete</button>
    <button onclick={startDetect} class="action-button assign-key">Detect Input Device</button>
    <button 
        onclick={startAssignMode} 
        class="action-button assign-key" 
        disabled={!selectedItemId || assignModeActive}
    >
        Assign Key
    </button>
    <button 
        onclick={cancelAssignMode} 
        class="action-button cancel"
        disabled={!assignModeActive}
    >
        Cancel
    </button>
</div>
<div style="text-align: center; display: flex; align-items: center; justify-content: center; gap: 20px;">
  <a target="_blank" rel="noopener noreferrer">
    <img src="{GitHubSvg}" style="width: 80px;" alt="GitHub" />
  </a>
  <ul style="list-style-type: none; padding: 0; margin: 0; text-align: left;">
    <li>• Report bugs</li>
    <li>• Leave a star</li>
  </ul>
</div>
            <div class="list-container">
                <ListBox onitem={itemClicked} Items={items} />
            </div>
        </div>
               <div class="main-content">
            {#if selectedItemId}
                <CodeMirror 
                    bind:value 
                    on:change={e => updateItemContent(e)} 
                    class="codemirror-container"
                    extensions={cmExtensions}
                />
            {:else}
                <div class="no-selection-message">
                    <p>No item selected</p>
                </div>
            {/if}
        </div>
    </div>
    
    {#if showDetectDialog}
        <DetectKeyboardDialog 
            onDetectDone={handleDetectDone} 
        />
    {/if}
    
<!-- Dialog component -->
  {#if $dialogState.visible}
    <MessageBoxDialog
      type={$dialogState.type}
      title={$dialogState.title}
      message={$dialogState.message}
      showInput={$dialogState.showInput}
      inputLabel={$dialogState.inputLabel}
      inputPlaceholder={$dialogState.inputPlaceholder}
      onResult={(result: any) => {
        $dialogState.visible = false;
        $dialogState.callback(result);
      }}
    />
  {/if}

    <!-- New Popup for Status Messages -->
    {#if statusMessage}
        <div class="status-popup-container">
            <div class="status-popup" class:show={showStatusPopup} class:active={assignModeActive}>
                {statusMessage}
            </div>
        </div>
    {/if}

    <!-- Instead of {$eventLuaError} -->
{#if $eventLuaError}
  <div class="error-container">
    <div class="error-box">
      <div class="error-header">
        <span class="error-title">Lua Error in: {JSON.parse($eventLuaError).itemName}</span>
      </div>
      <div class="error-content">
        <p class="error-message">{JSON.parse($eventLuaError).error}</p>
        <p class="error-timestamp">{JSON.parse($eventLuaError).timestamp}</p>
      </div>
    </div>
  </div>
{/if}
</main>

<style>

.app-container {
  display: flex;
  flex-direction: row;
  height: 100vh;
  width: 100%;
}

main {
  width: 100%;
  position: relative;
}

.sidebar {
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  width: 250px;
  border-right: 1px solid var(--border-color);
}

.no-selection-message {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;
  background-color: var(--bg-secondary);
  color: var(--text-tertiary);
  font-size: 1.2em;
  text-align: center;
  border-radius: 4px;
  border: 1px dashed var(--border-color);
  margin: 2px;
}

.no-selection-message p {
  padding: 20px;
  max-width: 80%;
}

/* Improved Styles for Action Buttons */
.action-buttons-container {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  padding: 12px;
}

.action-button {
  flex: 1 1 calc(50% - 6px);
  padding: 8px 6px;
  text-align: center;
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  transition: all 0.2s ease-in-out;
  font-size: var(--default_fontsize);
  position: relative;
  overflow: hidden;
  box-shadow: 0 1px 3px var(--shadow-color);
  cursor: pointer;
}

.action-button:hover:not(:disabled) {
  background-color: var(--accent-color-transparent);
  border-color: var(--accent-color);
}

.action-button:active:not(:disabled) {
  background-color: var(--accent-color);
  transform: translateY(0);
  box-shadow: 0 1px 2px var(--shadow-color);
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: var(--bg-secondary);
}

/* Special Styling for Primary Actions */
.action-button.primary {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color-dark);
}

.action-button.primary:hover:not(:disabled) {
  background-color: var(--accent-color-light);
  border-color: var(--accent-color);
}

.action-button.primary:active:not(:disabled) {
  background-color: var(--accent-color-dark);
}

/* For Key Assignment Buttons */
.action-button.assign-key {
  border-left: 3px solid var(--accent-color);
}

/* For Cancel Button */
.action-button.cancel {
  border-left: 3px solid var(--warning-color);
}

/* For Delete Button */
.action-button.delete {
  border-left: 3px solid var(--error-color);
}

/*Action Button end*/

/* Styles für die Popup-Statusnachrichten */
.status-popup-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 1000;
  pointer-events: none;
}

.status-popup {
  padding: 12px 16px;
  background-color: var(--bg-secondary);
  border-left: 4px solid var(--info-color);
  border-radius: 4px;
  box-shadow: 0 2px 8px var(--shadow-color);
  max-width: 300px;
  opacity: 0;
  transform: translateX(30px);
  transition: opacity 0.3s ease, transform 0.3s ease;
  color: var(--text-primary);
}

.status-popup.show {
  opacity: 1;
  transform: translateX(0);
}

.status-popup.active {
  background-color: var(--bg-tertiary);
  border-left-color: var(--warning-color);
  font-weight: bold;
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% { background-color: var(--bg-tertiary); }
  50% { background-color: var(--bg-secondary); }
  100% { background-color: var(--bg-tertiary); }
}

.list-container {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  overflow: auto;
margin: 8px;
}

.main-content {
  display: flex;
  flex-grow: 1;
  overflow: hidden;
}

:global(.codemirror-container) {
  width: 100%;
  margin: 2px;
  height: 100%;
}

:global(.cm-editor) {
  height: 100%;
}

/* Zusätzliche globale CodeMirror-Styles für bessere Dark-Mode-Darstellung */
:global(.cm-editor .cm-line) {
  font-family: 'Consolas', 'Monaco', 'Menlo', monospace;
  padding: 0 4px;
}

:global(.cm-editor .cm-scroller) {
  font-size: var(--default_fontsize);
  line-height: 1.5;
}

:global(.cm-editor .cm-content) {
  padding: 8px 0;
}

:global(.cm-editor .cm-gutters) {
  padding-right: 8px;
}

:global(.cm-editor .cm-tooltip) {
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

/* Verbesserte Sichtbarkeit der Auswahl */
:global(.cm-editor .cm-selectionBackground) {
  mix-blend-mode: multiply;
}

:global(.cm-editor.cm-focused .cm-selectionBackground) {
  mix-blend-mode: normal;
}

/* Fix für Light-Theme (falls der User wechselt) */
@media (prefers-color-scheme: light) {
  :global(.cm-editor) {
    background-color: var(--bg-secondary);
  }
  
  :global(.cm-editor .cm-content) {
    color: var(--text-primary);
  }
  
  :global(.cm-editor .cm-gutters) {
    background-color: var(--bg-primary);
    color: var(--text-tertiary);
  }
}

/* Zur style-Sektion hinzufügen */
.error-container {
  position: fixed;
  bottom: 20px;
  right: 20px;
  z-index: 1000;
  max-width: 450px;
}

.error-box {
  background-color: var(--bg-secondary);
  border-left: 4px solid #f44336; /* Rot für Fehler */
  border-radius: 4px;
  box-shadow: 0 3px 12px var(--shadow-color);
  overflow: hidden;
  animation: slide-in 0.3s ease-out;
}

.error-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 12px;
  background-color: rgba(244, 67, 54, 0.1);
  border-bottom: 1px solid rgba(244, 67, 54, 0.2);
}

.error-title {
  font-weight: bold;
  color: #f44336;
}


.error-content {
  padding: 12px;
}

.error-message {
  margin: 0 0 8px 0;
  font-family: 'Consolas', 'Monaco', 'Menlo', monospace;
  white-space: pre-wrap;
  word-break: break-word;
}

.error-timestamp {
  margin: 0;
  font-size: var(--default_fontsize);
  color: var(--text-tertiary);
}

@keyframes slide-in {
  from {
    transform: translateX(30px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
</style>