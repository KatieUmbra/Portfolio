import { backendRequest } from "$lib/backend/backend";
import type { BlogPost } from "$lib/backend/schema/blog";
import { Claims } from "$lib/backend/schema/user";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url, cookies }: any) => {
    const id = url.searchParams.get("id");

    const request = await backendRequest<BlogPost>(`blog/get?id=${id}`, {
        method: "GET",
        mode: "cors",
    });

    const userRequest = await backendRequest<Claims>("info", {}, { token: cookies.get("token"), currentPage: url.pathName });

    let currentUser: Claims | null = null;

    if (userRequest.isOk) {
        currentUser = userRequest.value;
    }

    if (request.isOk) {
        let post = request.value;
        let localTime: Date;
        if (typeof(post.creation) == "string") {
            const parsedDate = new Date(post.creation);
            localTime = new Date(parsedDate.getTime() - parsedDate.getTimezoneOffset()*60*1000);
        } else {
            localTime = new Date(post.creation.getTime() - post.creation.getTimezoneOffset()*60*1000);
        }
        return {
            post,
            currentUser,
            localTime
        }
    } else {
        return {
            post: {
                id: -1,
                title: "Oops! 404",
                description: `X_X post id=${id} not found`,
                content: "<p>Next time click on a post that exists!</p>",
                creation: "",
                likes: 404 },
                localTime: new Date("1404-04-04T0:00:00.000000Z")
            }
    }
}
