import type { ErrorResponse } from './ErrorResponse';
import type { OkResponse } from './OkResponse';

export type Response<T> = { type: "ok", data: OkResponse<T> } | { type: "err", error: ErrorResponse };