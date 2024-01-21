import type {PrismaClient} from "@prisma/client";
import type {Result} from "$lib/api/result";


export type UserWithPermissions = {
    username: string,
    permissions: { permission: string, level: number }[],
    password?: string,
}

export async function userWithPermissions(prisma: PrismaClient, username: string): Promise<Result<UserWithPermissions, never>> {
    const u = await prisma.user.findFirst({
        where: { username: username },
        include: { permissions: {
            select: { permission: true }
        } }
    });
    
    if (u == null) return { err: { message: "Username does not exist!", id: "username_does_not_exist"} };
    
    const permissions = u.permissions.map(p => { 
        return { permission: p.permission.permission, level: p.permission.level }});
    
    return { ok: {
        username: u.username,
        permissions: permissions,
        password: u.password,
    }};
}