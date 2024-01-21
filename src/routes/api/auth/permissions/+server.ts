import * as Endpoint from './endpoints';
import {authorize} from "$lib/api/auth";
import {userWithPermissions} from "$lib/controllers/user";
import { prisma } from '$lib/gen/db';

export async function GET({ url, request}) {
    const This = Endpoint.GET;
    
    const data = This.validateDataServer(url);
    if (!data.ok) return This.error(data.err);

    const result = await authorize(request, prisma, {
        required_level: 5,
        override_is_user: (username) => {
            return username === data.ok.username;
        }
    });
    if (result.err) return This.error(result.err);
    
    const user = await userWithPermissions(prisma, data.ok.username);
    if (user.err) return This.error(user.err);
    
    user.ok.password = undefined;
    
    return This.send(user.ok);
}