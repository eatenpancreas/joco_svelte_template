import { writable } from 'svelte/store';

export default writable<{ username: string, token: string } | undefined>(undefined)