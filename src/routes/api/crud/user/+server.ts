import * as Endpoint from './endpoints';
import {authorize} from "$lib/api/auth";
import { safeguard_async } from '$lib/api/result';
import { prisma } from '$lib/gen/db';

export async function DELETE({ url, request}) {
    const This = Endpoint.DELETE;
    
    const data = This.validateDataServer(url);
    if (!data.ok) return This.error(data.err);
    
    const result = await authorize(request, prisma, {
        required_level: 5,
        override_is_user: (username) => {
            return username === data.ok.username;
        }
    });
    if (result.err) return This.error(result.err);
    
    const user = await safeguard_async(async () => {
        return prisma.user.delete({
            where: {username: data.ok.username},
            include: {permissions: true}
        });
    });
    if (user.err) return This.error(user.err);
    
    return This.send({ user: user.ok, message: "User deleted!" });
}

export async function GET({ url, request}) {
    const This = Endpoint.GET;
    const data = This.validateDataServer(url);
    if (!data.ok) return This.error(data.err);
    
    const result = await authorize(request, prisma, {
        required_level: 5,
    });
    if (result.err) return This.error(result.err);
    
    const users = await safeguard_async(async () => {
        return prisma.user.findMany({
            include: {permissions: true},
            take: data.ok.limit ?? 10,
            skip: data.ok.offset ?? 0
        });
    });
    if (users.err) return This.error(users.err);
    
    const users_count = await safeguard_async(async () => {
        return prisma.user.count();
    });
    if (users_count.err) return This.error(users_count.err);
    
    let message = "Users found!";
    if (users.ok.length === 0) { message = "No users found!"; }
    
    let is_end = false;
    if (users.ok.length + (data.ok.offset ?? 0) >= users_count.ok) { is_end = true; }
    return This.send({ users: users.ok, message, is_end, total: users_count.ok });
}