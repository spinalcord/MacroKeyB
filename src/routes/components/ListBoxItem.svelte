<script lang="ts">
export let displayText: string = "(Empty)";
export let labelText: string = "(Empty)";
export let id: string = "123";
export let content: string = "text";
export let isSelected: boolean = false;
export let onitem: any; 

// Reactive declaration - recalculated when props change
$: itemData = { displayText, labelText, id, content, isSelected };

</script>


<div class="list-item-wrapper">
<button class="{isSelected ? "list-item-button-selected" : "list-item-button"}" on:click={() => onitem(itemData)}> 
        <div class="list-item-content"> 
            <div class="list-item-text">{displayText}</div>
            <div class="list-item-actions">
                <span class="list-item-tag">{labelText}</span>
            </div>
        </div>
</button>
</div>

<style>
    .list-item-wrapper {
        width: 100%;
        margin-bottom: 8px;
    }
    
    .list-item-button {
        display: flex;
        width: 100%;
        min-height: 48px;
        text-align: left;
        padding: 8px 12px;
        border: none;
        border-radius: 4px;
        background-color: var(--bg-tertiary);
        color: var(--text-secondary);
        cursor: pointer;
        transition: background-color 0.2s ease;
    }

    .list-item-button-selected {
        display: flex;
        width: 100%;
        min-height: 48px;
        text-align: left;
        padding: 8px 12px;
        border: none;
        border-radius: 4px;
        background-color: var(--hover-color);
        color: white; /* White font color for selected item */
        cursor: pointer;
        transition: background-color 0.2s ease;
    }

    .list-item-button:hover {
        background-color: var(--hover-color);
    }

    .list-item-content {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
    }

    .list-item-text {
        flex-grow: 1;
        font-size: var(--default_fontsize);
        padding-right: 12px;
        display: flex;
        align-items: center; /* Vertical centering */
        min-height: 32px; /* Ensure there is enough height */
    }

    .list-item-actions {
        flex-shrink: 0;
        display: flex;
        align-items: center; /* Vertical centering */
        justify-content: flex-end;
    }

    .list-item-tag {
        background-color: var(--accent-color-dark);
        color: white;
        border-radius: 4px;
        padding: 4px 8px;
        font-size: var(--default_fontsize);
        font-weight: 500;
        display: flex;
        align-items: center; /* Vertical centering */
        height: 100%; /* Use full height of parent element */
    }
</style>