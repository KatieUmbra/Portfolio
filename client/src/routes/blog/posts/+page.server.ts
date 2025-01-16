import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ url }) => {
    const id = url.searchParams.get("id");

    const req = await fetch(`http://localhost:8081/blog/get?id=${id}`, {
        method: "GET",
        mode: "cors",
        headers: {
            "Content-Type": "application/json",
        }
    });

    let json;

    try {
        json = await req.json();
        if (!req.ok) {
            return { post: {
                id: -1,
                title: "Oops! 404",
                description: `X_X post id=${id} not found`,
                content: "<p>Next time click on a post that exists!</p>",
                creation: "1404-04-04T16:00:00.000000Z",
                likes: 404 } }
        }
    } catch (e) {
        if (!req.ok) {
            return { post: {
                id: -1,
                title: "Oops! 404",
                description: `X_X post id=${id} not found`,
                content: "<p>Next time click on a post that exists!</p>",
                creation: "1404-04-04T0:00:00.000000Z",
                likes: 404 } }
        }
    }

    return { post: json };
}
