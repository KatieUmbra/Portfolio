import { error } from "@sveltejs/kit";
import type { PageServerLoad } from "./$types"
import { backendRequest } from "$lib/backend/backend";
import type { BlogPost } from "$lib/backend/schema/blog";


export const load: PageServerLoad = async (_) => {
    const request = await backendRequest<{ vec: BlogPost[] }>("blog/get_latest?amount=10", {
        method: "GET",
        mode: "cors",
        headers: {
            "Content-Type": "application/json"
        }
    });

    if (request.isOk) {
        let posts = request.value.vec;
        posts.forEach((element: BlogPost) => {
            const parsedDate = new Date(element.creation);
            const localTime = new Date(parsedDate.getTime() - parsedDate.getTimezoneOffset()*60*1000);
            element.creation = localTime;
        });
        return { posts };
    } else {
        error(request.error.status_code, request.error);
    }
}
