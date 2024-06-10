<script lang="typescript">
    let username = "Guest";
    let token = "";

    function onSubmit(e) {
        const formData = new FormData(e.target);

        const data = {};
        for (let field of formData) {
            const [key, value] = field;
            console.log(value);
            data[key] = value;
        }
        console.log(data);
    }
</script>

<div class="login">
    <form on:submit|preventDefault={ onSubmit }>
        <h2>Login</h2>
        <input type="text" placeholder="Username"/>
        <input type="password" placeholder="Password"/>
        <button type="submit">Submit</button>
    </form>
</div>

<div class="register">
    <form>
        <h2>Register</h2>
        <input type="text" placeholder="Username"/>
        <input type="text" placeholder="Display Name"/>
        <input type="email" placeholder="Email"/>
        <input type="password" placeholder="Password"/>
        <button>Submit</button>
    </form>
</div>
