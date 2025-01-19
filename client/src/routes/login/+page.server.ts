import { backendRequest } from "$lib/backend/backend";
import { LoginData } from "$lib/backend/schema/user";
import { fail } from "@sveltejs/kit";

export const actions = {
    default: async ({ cookies, request }: any) => {
        const data = await request.formData();

        const loginData = new LoginData(
            data.get("username"),
            data.get("password"),
        );

        const loginRequest = await backendRequest<{ token: string }>("login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(loginData),
        });

        if (loginRequest.isOk) {
            const jwt = loginRequest.value;
            // remove `maxAge` to make the cookie non persistent
            cookies.set("token", jwt.token, { path: "/", maxAge: 86400 });
        } else {
            return fail(loginRequest.error.status_code, { ...loginRequest.error, failure: true });
        }

        return { failure: false };
    },
};
