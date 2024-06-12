import { fail } from "@sveltejs/kit";

export const actions = {
    default: async ({ cookies, request }) => {

        const data = await request.formData();

        const formData = {
                "username": data.get("username"),
                "password": data.get("password")
        };

        const req = await fetch("http://localhost:8080/login", {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(formData)
        }); 

        let json = await req.json();
        if (!req.ok) {
            return fail(req.status, {...json, failure: true });
        }
        const jwt = await json;

        cookies.set("token", jwt.token, {path:"/", maxAge: 86400});
        return { failure: false };
    },
}
