export async function load({ url, cookies }) {
    console.log("verifying token...");
    const auth_token = cookies.get("token");
    const verification_token = {
        veri_token: url.searchParams.get("token"),
    };

    const request = await fetch("http://192.168.1.20:8081/verify", {
        method: "PUT",
        headers: {
            Authorization: "Bearer " + auth_token,
            "Content-Type": "application/json",
        },
        body: JSON.stringify(verification_token),
    })

    const info = await request.text();
    const status =  request.status;

    return { status };
}
