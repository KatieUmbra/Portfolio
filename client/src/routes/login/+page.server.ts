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
        const jwt = await req.text();
        cookies.set("token", jwt, {path:"/", maxAge: 86400});
    },
}
