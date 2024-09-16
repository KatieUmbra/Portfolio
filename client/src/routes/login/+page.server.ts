import { fail } from "@sveltejs/kit";

export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();

        const formData = {
            username: data.get("username"),
            password: data.get("password"),
        };

        const req = await fetch("http://192.168.1.20:8081/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData),
        });

        console.log(req.body);

        let json = await req.json();
        if (!req.ok) {
            return fail(req.status, { ...json, failure: true });
        }
        const jwt = await json;

        // remove `maxAge` to make the cookie non persistent
        cookies.set("token", jwt.token, { path: "/", maxAge: 86400 });
        return { failure: false };
    },
};
