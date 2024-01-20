import {type Writable, writable} from "svelte/store";
import {z} from "zod";

type ClientAuth = { jwt: string, username: string }
export const client_auth: Writable<ClientAuth | null> = writable(null);

export function init_auth() {
    const jwt = localStorage.getItem("jwt");
    const username = localStorage.getItem("username");
    if (jwt && username) {
        client_auth.set({jwt, username});
    }
}

export const Auth = z.object({
    message: z.string(),
    username: z.string(),
    token: z.string(),
});