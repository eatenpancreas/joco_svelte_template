
import { type EndpointError } from '$lib/api/EndpointError';

export type Result<T, E> = { err: EndpointError<E>, ok?: undefined } | { ok: T, err?: undefined };


export function is_exception(unsafe_operation: () => any) {
    try { unsafe_operation(); return false; } 
    catch (e) { return true; }
}
export async function is_exception_async(unsafe_operation: () => Promise<any>) {
    try { await unsafe_operation(); return false; }
    catch (e) { return true; }
}


export function safeguard<T, E>(unsafe_operation: () => T): Result<T, E> {
    try { return { ok: unsafe_operation() } } 
    catch (e: any) { return handle_err(e); }
}

export async function safeguard_async<T, E>(unsafe_operation: () => Promise<T>): Promise<Result<T, E>> {
    try { return { ok: await unsafe_operation() } }
    catch (e: any) { return handle_err(e); }
}

function handle_err(e: any): {err: {message: string}} {
    if (e.message != undefined && typeof e.message === "string") {
        return { err: { message: e.message } }
    } else {
        try { return { err: { message: "An undefined error occurred! On " + JSON.stringify(e) } } }
        catch { return { err: { message: "An undefined error occurred! ???" } } }
    }
}