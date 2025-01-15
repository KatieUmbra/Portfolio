import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types"


export const load: PageServerLoad = async (_) => {
    const req = await fetch("http://localhost:8081/blog/get_latest?amount=10", {
        method: "GET",
        mode: "cors",
        headers: {
            "Content-Type": "application/json"
        }
    });

    let json;
    
    try {
        json = await req.json();
        if (!req.ok) {
            error(500, {message: "There has been an error"});
        }
    } catch (e) {
        if (!req.ok) {
            error(500, {message: "There has been an error"});
        }
    }

    return { posts: json.vec };
}
