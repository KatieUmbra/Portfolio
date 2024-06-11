export const actions = {
    default: async ({ cookies, request }) => {
        const data = await request.formData();
        console.log(data);
        const formData = {
            "username": data.get("username"),
            "display_name": data.get("displayName"),
            "email": data.get("email"),
            "password": data.get("password")
        };
        console.log(formData);
        const req = await fetch("http://localhost:8080/register", {
            method: "POST",
            mode: "cors",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(formData)
        });
        console.log(req);
    }
}
