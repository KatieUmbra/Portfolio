import { backendRequest } from "$lib/backend/backend";
import type { Claims } from "$lib/backend/schema/user";

export async function load({ cookies }: any) {
    const request = await backendRequest<Claims>("info", {
        method: "GET",
    }, cookies.get("token"));

    if (request.isOk) {
        const info = request.value;
        return { info };
    }
}
