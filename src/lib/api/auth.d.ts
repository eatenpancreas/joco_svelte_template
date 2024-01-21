
export type RequiredAuths = {
    required_level?: number;
    required_permissions?: {
        permission: string;
        or_level?: number;
    }[];
    override_is_user?: (username: string) => boolean;
}