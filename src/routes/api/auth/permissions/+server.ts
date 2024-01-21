import * as This from './endpoints';
import {authorize} from "$lib/api/auth";
import {PrismaClient} from "@prisma/client";
import {userWithPermissions} from "$lib/controllers/user";

export async function GET({ url, request}) {
    const data = This.GET.validateDataServer(url);
    if (!data.ok) return This.GET.error(data.err);

    const prisma = new PrismaClient();
    This.GET.dbInit(prisma);

    const result = await authorize(request, prisma, {
        required_level: 5,
        override_is_user: (username) => {
            return username === data.ok.username;
        }
    });
    
    if (result.err) return This.GET.error(result.err);
    const user = await userWithPermissions(prisma, data.ok.username);
    if (user.err) return This.GET.error(user.err);
    
    return This.GET.trySend(user.ok);
}