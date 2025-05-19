<script lang="ts">
    import ListBoxItem from "./ListBoxItem.svelte";
    import type { ListBoxItemData } from "./ListBoxItemData";
    export let Items: Array<ListBoxItemData> = [];
    export let onitem: any;
    
    // Search state
    let nameSearchTerm: string = "";
    let labelSearchTerm: string = "";
    
    // Filtered items based on search terms
    $: filteredItems = Items.filter(item => {
        const displayTextMatch = item.displayText.toLowerCase().includes(nameSearchTerm.toLowerCase());
        const labelTextMatch = item.assignedKey.toLowerCase().includes(labelSearchTerm.toLowerCase());
        
        // If both search terms are provided, both must match
        if (nameSearchTerm && labelSearchTerm) {
            return displayTextMatch && labelTextMatch;
        }
        
        // If only one search term is provided, that one must match
        if (nameSearchTerm) {
            return displayTextMatch;
        }
        
        if (labelSearchTerm) {
            return labelTextMatch;
        }
        
        // If no search terms are provided, show all items
        return true;
    });
</script>
<div class="list-box">
    <div class="search-container">
        <div class="search-field">
            <label for="nameSearch">Name</label>
            <input 
                id="nameSearch"
                type="text" 
                placeholder="Search name..." 
                bind:value={nameSearchTerm}
                class="search-input"
            />
        </div>
        <div class="search-field">
            <label for="labelSearch">Key</label>
            <input 
                id="labelSearch"
                type="text" 
                placeholder="Search key..." 
                bind:value={labelSearchTerm}
                class="search-input"
            />
        </div>
    </div>
    
    <div class="list-content">
        {#if filteredItems.length > 0}
            {#each filteredItems as item}
                <ListBoxItem 
                    onitem={(itemData: any) => {
                        onitem(itemData);
                    }} 
                    labelText={item.assignedKey} 
                    isSelected={item.isSelected} 
                    displayText={item.displayText}
                    content={item.content}
                    id={item.id}
                />
            {/each}
        {:else}
            <div class="no-results">No items match your search</div>
        {/if}
    </div>
</div>

<style>
    .list-box {
        width: 100%;
        background-color: var(--bg-primary);
        display: flex;
        flex-direction: column;
    }
    
    .search-container {
        display: flex;
        gap: 12px;
        margin-bottom: 16px;
        padding: 12px;
        background-color: var(--bg-secondary);
        border-radius: 4px;
    }
    
    .search-field {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 4px;
    }
    
    .search-field label {
        font-size: var(--small-fontsize, 12px);
        color: var(--text-secondary);
        margin-bottom: 2px;
    }
    
    .search-input {
        border-radius: 4px;
        border: 1px solid var(--border-color, #ddd);
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        font-size: var(--default_fontsize);
        width: 100%;
    }
    
    .search-input:focus {
        outline: none;
        border-color: var(--accent-color);
    }
    
    .list-content {
        overflow-y: auto;
        max-height: 500px; 
    }
    
    .no-results {
        padding: 16px;
        text-align: center;
        color: var(--text-secondary);
        font-style: italic;
        background-color: var(--bg-tertiary);
        border-radius: 4px;
    }
</style>