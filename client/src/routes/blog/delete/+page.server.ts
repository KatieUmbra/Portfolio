import { backendRequest } from "$lib/backend/backend";
import { redirect } from "@sveltejs/kit";
import type { PageServerLoad } from "../$types";

export const load: PageServerLoad = async ({ cookies, url }: any) => {
    const id = url.searchParams.get("id");

    if (id == undefined || id == null) {
        throw redirect(303, "/blog");
    }

    let deleteRequest = await backendRequest(`blog/delete?id=${id}`, {
        method: "DELETE",
        mode: "cors",
    }, { token: cookies.get("token"), currentPage: url.pathName });

    if (deleteRequest.isOk) {
        throw redirect(303, "/blog");
    } else {
        console.log(deleteRequest.error);
        throw redirect(303, `/blog/post?id=${id}`);
    }
};
