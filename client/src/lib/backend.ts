import { Result } from "@badrap/result"

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

export async function backendRequest<T>(route: string, details?: RequestInit): Promise<Result<T, ApiError>> {
    const request = await fetch("http://localhost:8081/" + route, details);

    let json;

    try {
        json = await request.json();
        if (!request.ok) {
            return Result.err(json);
        }
    } catch (_) {
        if (!request.ok) {
            return Result.err(new ApiError(request.status, -1, ""));
        }
    }

    return Result.ok(json as T);
}
