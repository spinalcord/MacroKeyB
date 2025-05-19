// dialogManager.ts
import { writable } from 'svelte/store';

type DialogType = "yes/no/cancel" | "yes/no" | "ok/cancel" | "ok" | "input";
type DialogResult = "yes" | "no" | "ok" | "cancel" | string;

interface DialogOptions {
  type: DialogType;
  title: string;
  message: string;
  showInput?: boolean;
  inputLabel?: string;
  inputPlaceholder?: string;
}

interface DialogState extends DialogOptions {
  visible: boolean;
  callback: (result: string) => void;
}

// Create a writable store with the dialog state
const dialogState = writable<DialogState>({
  visible: false,
  type: "ok",
  title: "",
  message: "",
  showInput: false,
  inputLabel: "",
  inputPlaceholder: "",
  callback: () => {}
});

/**
 * Shows a dialog with the specified options
 * @param type Type of dialog ("yes/no/cancel", "yes/no", "ok/cancel", "ok", "input")
 * @param title Dialog title
 * @param message Dialog message
 * @param callback Function to call with the result
 * @param inputOptions Optional input field options
 */
export function showDialog(
  type: DialogType,
  title: string,
  message: string,
  callback: (result: DialogResult) => void,
  inputOptions?: {
    label?: string;
    placeholder?: string;
  }
) {
  const isInput = type === "input";
  
  dialogState.set({
    visible: true,
    type: isInput ? "ok/cancel" : type, // Input type uses ok/cancel buttons
    title,
    message,
    showInput: isInput,
    inputLabel: inputOptions?.label || "",
    inputPlaceholder: inputOptions?.placeholder || "",
    callback
  });
}

/**
 * Closes the currently open dialog
 * @param result Result to pass to the callback
 */
export function closeDialog(result: DialogResult = "cancel") {
  dialogState.update(state => {
    if (state.visible) {
      state.callback(result);
    }
    return { ...state, visible: false };
  });
}

// Helper functions for common dialog types
export function showAlert(message: string, title = "Alert", callback: () => void = () => {}) {
  showDialog("ok", title, message, callback);
}

export function showConfirm(message: string, title = "Confirm", callback: (confirmed: boolean) => void) {
  showDialog("yes/no", title, message, result => {
    callback(result === "yes");
  });
}

export function showPrompt(
  message: string, 
  title = "Input", 
  callback: (value: string | null) => void,
  options?: { 
    label?: string; 
    placeholder?: string;
    defaultValue?: string;
  }
) {
  showDialog("input", title, message, result => {
    if (result.startsWith("ok:")) {
      // Extract the input value after "ok:"
      callback(result.substring(3));
    } else {
      // Cancel was clicked
      callback(null);
    }
  }, {
    label: options?.label,
    placeholder: options?.placeholder
  });
}

export { dialogState };