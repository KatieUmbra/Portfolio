import { backendRequest, preemptiveAuthCheck } from "$lib/backend/backend";
import type { Claims } from "$lib/backend/schema/user";

export async function load({ cookies, url }: any) {

    await preemptiveAuthCheck({ url, cookies });

    const request = await backendRequest<Claims>("info", {
        method: "GET",
    }, { token: cookies.get("token"), currentPage: url.pathname });

    if (request.isOk) {
        const info = request.value;
        return { info };
    }
}
