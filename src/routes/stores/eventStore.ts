// Events: Backend -> Frontend
import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

export const eventLuaExecuted = writable<string>('');
export const eventLuaError = writable<string>('');

listen('lua-execution', (event) => {
  // Type-Assertion
    //eventLuaError.set((event.payload as { message: string }).message);
    eventLuaError.set("");
});

listen('lua-error', (event) => {
  // Type-Assertion
  eventLuaError.set((event.payload as { message: string }).message);
});

