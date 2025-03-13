import { Claims } from "$lib/backend/schema/user";
import type { Actions, PageServerLoad } from "./$types";
import { fail } from "@sveltejs/kit";
import { backendRequest } from "$lib/backend/backend";
import { BlogComment, BlogCommentData, type BlogPost } from "$lib/backend/schema/blog";

export const load: PageServerLoad = async ({ url, cookies }: any) => {
    const id = url.searchParams.get("id");

    const commentsRequest = await backendRequest<{ vec: BlogComment[] }>(`blog/comment/get_latest?post_id=${id}&page=1&amount=10`, {
        method: "GET",
        mode: "cors"
    });

    let comments: BlogComment[] | null = null;
    if (commentsRequest.isOk) {
        comments = commentsRequest.value.vec;
    }

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
            comments,
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

export const actions = {
    default: async({ request, cookies, url }: any) => {
        const data = await request.formData();
        const token = cookies.get("token");

        const formData = new BlogCommentData(
            data.get("comment") as string,
            parseInt(url.searchParams.get("id")),
            null
        );
        const req = await backendRequest("blog/comment/post", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData)
        }, { token, currentPage: url.pathname });

        if (!req.isOk) {
            return fail(req.error.status_code, { ...req.error, failure: true });
        }

    }
} satisfies Actions;
