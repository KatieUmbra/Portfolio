import { fail, redirect } from "@sveltejs/kit";
import type {Actions} from "./$types";
import type { PageServerLoad } from "../$types";
import { backendRequest, preemptiveAuthCheck } from "$lib/backend/backend";
import { BlogPostData, type BlogPost } from "$lib/backend/schema/blog";

let editPost = {
    edit: false,
    id: 0
}

export const load: PageServerLoad = async ({ url, cookies }) => {
    editPost.edit = false;
    editPost.id = 0;
    const id = url.searchParams.get("edit");

    await preemptiveAuthCheck({ url, cookies });

    if (id != null && id != undefined) {
        const request = await backendRequest<BlogPost>(`blog/get_md?id=${id}`, { method: "GET" });

        if (request.isOk) {
            editPost.id = parseInt(id as string);
            editPost.edit = true;
            return { status: 200, post: request.value };
        }
    }

};

export const actions = {
    default: async ({ request, cookies, url }: any) => {
        const data = await request.formData();
        const token = cookies.get("token");

        const formData = new BlogPostData(
            data.get("title") as string,
            data.get("description") as string,
            data.get("content") as string
        );

        let method = "POST";
        let requestStr = "blog/post";
        if (editPost.edit) {
            method = "PUT";
            requestStr = `blog/edit?id=${editPost.id}`;
        }

        const req = await backendRequest(requestStr, {
            method: method,
            mode: "cors", headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData)
        }, { token, currentPage: url.pathname });

        if (req.isOk) {
            if (editPost.edit) {
                redirect(303, `/blog/post?id=${editPost.id}`);
            } else {
                redirect(303, `/blog`);
            }
        } else {
            return fail(req.error.status_code, { ...req.error, failure: true });
        }

    }
} satisfies Actions;
