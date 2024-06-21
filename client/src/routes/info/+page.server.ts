export async function load({cookies}) {
    const token = cookies.get("token");

    const request = await fetch("http://localhost:8081/info", {
        method: "GET",
        "headers": {
            "Authorization": "Bearer " + token,
        }
    })

    const info = await request.text();

    return { info };
}
