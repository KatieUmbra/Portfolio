import { fail } from "@sveltejs/kit";

export const actions = {
    default: async ({ request }) => {
        const data = await request.formData();

        const formData = {
            username: data.get("username"),
            display_name: data.get("displayName"),
            email: data.get("email"),
            password: data.get("password"),
            verify: data.get("verifyPassword")
        };

        if (formData.password != formData.verify) {
            return fail(500, {message: "Passwords do not match!", failure: true})
        }

        const req = await fetch("http://localhost:8081/register", {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        });

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
    },
};
