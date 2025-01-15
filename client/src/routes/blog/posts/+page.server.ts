import { error, fail } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url }) => {
    const id = url.searchParams.get("id");

    const req = await fetch(`http://localhost:8081/blog/get?id=${id}`, {
        method: "GET",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        }
    });

    let json;

    try {
        json = await req.json();
        if (!req.ok) {
            error(404, {message: "Post couldn't be found."});
        }
    } catch (e) {
        if (!req.ok) {
            error(404, {message: "Post couldn't be found."});
        }
    }

    return { post: json };
}
