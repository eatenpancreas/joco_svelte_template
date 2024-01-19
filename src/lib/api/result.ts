
import { type EndpointError } from '$lib/api/EndpointError';

export type Result<T, E> = { err: EndpointError<E>, ok?: undefined } | { ok: T, err?: undefined };