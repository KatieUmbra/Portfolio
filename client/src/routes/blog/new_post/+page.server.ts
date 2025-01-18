import { fail, redirect } from "@sveltejs/kit";
import type {Actions} from "./$types";
import type { PageServerLoad } from "../$types";

let blogRequest = "post";
let editId = 0;

export const load: PageServerLoad = async ({ url }) => {
    const id = url.searchParams.get("edit");

    if (id !== undefined) {

        const req = await fetch(`http://localhost:8081/blog/get_md?id=${id}`, {
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
                return { status: req.status, post: { ...json } };
            }
        } catch (e) {
            if (!req.ok) {
                return { status: req.status };
            }
        }

        editId = parseInt(id as string);
        blogRequest = "edit";
        return { status: 200, post: { ...json } };
    }

};

export const actions = {
    default: async ({ request, cookies }) => {
        const data = await request.formData();
        const token = cookies.get("token");

        const formData = {
            title: data.get("title"),
            description: data.get("description"),
            content: data.get("content")
        }

        let method = "POST";
        let requestStr = "http://localhost:8081/blog/post";
        if (blogRequest = "edit") {
            method = "PUT";
            requestStr = `http://localhost:8081/blog/edit?id=${editId}`;
        }
        const req = await fetch(requestStr, {
            method: method,
            mode: "cors", headers: {
                "Content-Type": "application/json",
                Authorization: "Bearer " + token,
            },
            body: JSON.stringify(formData)
        })

        try {
            let json = await req.json();
            if (!req.ok) {
                return fail(req.status, { ...json, failure: true });
            }
        } catch (e) {
            if (!req.ok) {
                return fail(req.status, { failure: true });
            }
        }

        return { failure: false };
    }
} satisfies Actions;
