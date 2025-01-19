import { redirect } from "@sveltejs/kit";

export async function load({ url, cookies }: any) {
    let token = decodeURI(url.searchParams.get("token"));
    let bounce = decodeURI(url.searchParams.get("bounce"));
    let oldToken = cookies.get("token");
    if (oldToken != null) {
        if (oldToken != token) {
            cookies.set("token", token, { path: "/", maxAge: 86400 });
        }
    }
    if (bounce.includes("?")) {
        bounce += "&up=1";
    } else {
        bounce += "?up=1";
    }
    redirect(308, bounce);
}
