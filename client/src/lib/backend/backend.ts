import { Result } from "@badrap/result"
import { redirect } from "@sveltejs/kit";
import type { Claims } from "./schema/user";

export class ApiError implements Error {
    status_code: number;
    error_code: number;
    message: string;
    name: string;

    constructor(status_code: number, error_code: number, message: string) {
        this.status_code = status_code;
        this.error_code = error_code;
        this.message = message;
        this.name = "";
    }
}

export async function preemptiveAuthCheck({ url, cookies }: any) {
    const noRed = url.searchParams.get("up");

    // preemptively updates user cookie so they dont lose their post draft is their cookie changes during the writing process
    // this is duct tape but if it works it works (╥﹏╥)
    if (noRed == null || noRed == "1") {
        const token = cookies.get("token");
        if (token != null) {
            await backendRequest<Claims>("info", {
                method: "GET",
            }, { token, currentPage: url.pathname });
        }
    }
}

export async function backendRequest<T>(route: string, details?: RequestInit, auth?: { token: string, currentPage: string }): Promise<Result<T, ApiError>> {
    if (auth != null) {
        if (details?.headers == null) {
            (details as any).headers = {}
        }
        (details?.headers as any).Authorization = "Bearer " + auth?.token;
    }
    const request = await fetch("http://localhost:8081/" + route, details);
    let json;
    try {
        json = await request.json();
        if (!request.ok) {
            return Result.err(json);
        }
    } catch (_) {
        if (!request.ok) {
            console.log(`is ok?: ${request.ok}`);
            if (auth != null && request.status == 409) {
                console.log(`Auth: ${auth == null}, Request: ${request.status}`);
                // HANDLE JWT EXPIRED
                const jwtRequest = await backendRequest<{ token: string }>("refreshJwt", {
                    method: "POST",
                    mode: "cors",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify({ token: auth?.token })
                });
                if (jwtRequest.isOk) {
                    const bounce = encodeURI(auth?.currentPage);
                    const encToken = encodeURI(jwtRequest.value.token);
                    return redirect(303, `/update?bounce=${bounce}&token=${encToken}`);
                } else {
                    return redirect(303, "/login");
                }
            }
                return Result.err(new ApiError(request.status, -1, ""));
            }
        }
    return Result.ok(json as T);
}
