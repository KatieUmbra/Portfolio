import { backendRequest } from "$lib/backend/backend";
import { UserData } from "$lib/backend/schema/user";
import { fail } from "@sveltejs/kit";

export const actions = {
    default: async ({ request }: any) => {
        const data = await request.formData();

        const userData = new UserData (
            data.get("username")as string,
            data.get("displayName") as string,
            data.get("email") as string,
            data.get("password"),
        );

        if (userData.password != data.get("verifyPassword") as string) {
            return fail(500, {message: "Passwords do not match!", failure: true})
        }

        const registerRequest = await backendRequest("register", {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(userData),
        })

        if (registerRequest.isErr) {
            return fail(registerRequest.error.status_code, { ...registerRequest.error, failure: true });
        }

        return { failure: false };
    },
};
