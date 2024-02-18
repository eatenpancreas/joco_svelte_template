import { writable } from 'svelte/store';

// TODO: Cookie
export default writable<{ username: string, token: string } | undefined>(undefined)