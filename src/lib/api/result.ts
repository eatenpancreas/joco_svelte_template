
import { type EndpointError } from '$lib/api/EndpointError';

export type Result<T, E> = { err: EndpointError<E>, ok?: undefined } | { ok: T, err?: undefined };


export function errMsg<T, E>(message: string, id: string, location?: string): Result<T, E> {
    return { err: { message, id, location } };
}


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

export function handle_err(e: any): Result<any, any> {
    if (e.message != undefined && typeof e.message === "string") {
        return errMsg(e.message, "unk")
    } else {
        try { return errMsg("An undefined error occurred!", "unk", JSON.stringify(e)) }
        catch { return errMsg("An undefined error occurred! ???", "unk") }
    }
}