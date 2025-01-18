import { fail, redirect } from "@sveltejs/kit";
import type {Actions} from "./$types";
import type { PageServerLoad } from "../$types";
import { backendRequest } from "$lib/backend";

class BlogPost {
    constructor(
        public id: number,
        public creator: string,
        public content: string,
        public description: string,
        public title: string,
        public creation: Date,
        public likes: number) {
    }
}

class FormData {
    constructor(
        public title: string,
        public description: string,
        public content: string
    ) {}
}

let editPost = {
    edit: false,
    id: 0
}

export const load: PageServerLoad = async ({ url }) => {
    const id = url.searchParams.get("edit");

    if (id !== undefined) {
        const request = await backendRequest<BlogPost>(`blog/get_md?id=${id}`);

        if (request.isOk) {
            editPost.id = parseInt(id as string);
            editPost.edit = true;
            return { status: 200, post: request.value };
        }
    }

};

export const actions = {
    default: async ({ request, cookies }) => {
        const data = await request.formData();
        const token = cookies.get("token");

        const formData = new FormData(
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
                Authorization: "Bearer " + token,
            },
            body: JSON.stringify(formData)
        });

        if (req.isOk) {
            if (editPost.edit) {
                redirect(303, `/blog/post?id=${editPost.id}`);
            }
            return { failure: false };
        } else {
            return fail(req.error.status_code, { ...req.error, failure: true });
        }

    }
} satisfies Actions;
