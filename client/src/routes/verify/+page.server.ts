import {fail} from "@sveltejs/kit";

export async function load({ url, cookies }) {
    const auth_token = cookies.get("token");
    const verification_token = {
        veri_token: url.searchParams.get("token"),
    };

    const request = await fetch("http://localhost:8081/verify", {
        method: "PUT",
        mode: "cors",
        headers: {
            Authorization: "Bearer " + auth_token,
            "Content-Type": "application/json",
        },
        body: JSON.stringify(verification_token),
    })

    let json = await request.json();
    if (!request.ok) {
        return fail(request.status, { ...json, failure: true });
    }
    const jwt = await json;

    cookies.set("token", jwt.token, { path: "/" });
    const info = "Your account has been verified!";

    return { info };
}
