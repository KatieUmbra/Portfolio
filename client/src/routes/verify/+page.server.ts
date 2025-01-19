import { backendRequest } from "$lib/backend/backend";
import {fail, redirect} from "@sveltejs/kit";

export async function load({ url, cookies }: any) {
    const token = cookies.get("token");
    const veriToken = {
        veri_token: url.searchParams.get("token"),
    };

    const verifyRequest = await backendRequest<{ token: string }>("verify", {
        method: "PUT",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(veriToken)
    }, { token, currentPage: url.pathName });

    if (verifyRequest.isOk) {
        cookies.set("token", verifyRequest.value.token, { path: "/" });
    } else {
        return fail(verifyRequest.error.status_code, { ...verifyRequest.error, failure: true });
    }

    return redirect(303, "/");
}
