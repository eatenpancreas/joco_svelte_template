import jwt from "jsonwebtoken";
import type {Result} from "$lib/api/result";

export async function asyncSign(payload: string | object | Buffer, secret: jwt.Secret, options: jwt.SignOptions): Promise<Result<string, any>> {
    return new Promise((resolve) => {
        jwt.sign(payload, secret, options, (err, token) => {
            if (err) resolve({ err: err });
            if (!token) resolve({ err: { message: "Token failed to be created!" } });
            else resolve({ ok: token });
        });
    });
}