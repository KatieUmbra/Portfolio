import { fail } from "@sveltejs/kit";
import type {Actions} from "./$types";

export const actions = {
    default: async ({ request, cookies }) => {
        const data = await request.formData();
        const token = cookies.get("token");

        const formData = {
            title: data.get("title"),
            description: data.get("description"),
            content: data.get("content")
        }

        const req = await fetch("http://localhost:8081/blog/post", {
            method: "POST",
            mode: "cors",
            headers: {
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
