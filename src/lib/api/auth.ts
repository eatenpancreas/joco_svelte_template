/* eslint-disable @typescript-eslint/no-explicit-any */

import type {RequiredAuths} from './auth.d';
import jwt from "jsonwebtoken";
import {env} from "$env/dynamic/private";
import { errMsg, type Result, safeguard, safeguard_async } from '$lib/api/result';
import type {PrismaClient} from "@prisma/client";
import pkg from 'bcryptjs';
import {userWithPermissions, type UserWithPermissions} from "$lib/controllers/user";
import { prisma } from '$lib/gen/db';
const {compare} = pkg;

export async function authorize(request: Request, prisma: PrismaClient, required?: RequiredAuths): Promise<Result<UserWithPermissions, any>> {
    const raw_token = request.headers.get("Authorization");
    
    if (!raw_token) return errMsg("Missing Authorization header", "auth")
    if (!raw_token.startsWith("Bearer ")) return errMsg("Invalid Authorization header, needs to be Bearer", "auth_no_bearer")
    const auth = raw_token.slice(7, raw_token.length);

    const secret = env.JWT_SECRET;
    if (!secret) return errMsg("Environments are not correct!", "env")

    const token_verify = safeguard(() => jwt.verify(auth, secret));
    if (token_verify.err) return { err: token_verify.err };
    
    const token_data = token_verify.ok;
    if (typeof token_data === "string") return errMsg("Token data is not an object!", "token_data_not_object");
    const token = token_data as { username: string, password: string };
    if (!token.username) return errMsg("Token has no username!", "token_no_username");
    if (!token.password) return errMsg("Token has no password!", "token_no_password");
    
    const u = await userWithPermissions(prisma, token.username);
    if (u.err) return { err: u.err };
    if (!u.ok.password) return errMsg("User has no password!", "u_no_pass");
    const pass_is_correct = await compare(token.password, u.ok.password);
    if (!pass_is_correct) return errMsg("Incorrect password!", "incorrect_pass");
    
    const verif = await safeguard_async(() => prisma.unverifiedUser.findFirst({ where: { user_id: u.ok.id } }));
    if (verif.ok && JSON.stringify(verif.ok) !== "{}") {
        return errMsg("User is not verified!", "user_not_verified");
    }

    if (!isAuthorized(u.ok, token.username, required)) return errMsg(`User ${token.username} not authorized!`, "auth");

    return u;
}

export function isAuthorized(user: UserWithPermissions, requestingUsername: string, required?: RequiredAuths): boolean {
    if (required) {
        if (required.override_is_user) {
            if (required.override_is_user(requestingUsername)) return true;
        }

        if (required.required_permissions) {
            const perms = user.permissions.map(p => p.permission);

            const max_level = Math.max(...user.permissions.map(p => p.level));
            if (!required.required_permissions.every(p => perms.includes(p.permission) || (p.or_level && max_level >= p.or_level)))
                return false;
        } else if (required.required_level){
            const req = required.required_level;
            const levels = user.permissions.map(p => p.level);
            if (!levels.some(l => l >= req)) return false;
        }
    }
    
    return true;
}