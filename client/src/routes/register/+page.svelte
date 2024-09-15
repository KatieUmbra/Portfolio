<script lang="ts">
    import "../../app.css";
    import type { ActionData } from "./$types";

    export let form: ActionData;
</script>

<div
    class="gird-cols-1 bg-95 grid min-h-screen w-full items-center justify-center gap-2">
    <form class="grid bg-white" action="/register" method="POST">
        <p class="m-3 text-xl font-bold">Register</p>
        <input
            type="text"
            class="txt-in95 m-3"
            name="username"
            placeholder="Username" />
        <input
            type="text"
            class="txt-in95 m-3"
            name="displayName"
            placeholder="Display Name" />
        <input
            type="email"
            class="txt-in95 m-3"
            name="email"
            placeholder="Email" />
        <input
            type="password"
            class="txt-in95 m-3"
            name="password"
            placeholder="Password" />
        <button type="submit" class="btn95 m-3 max-w-min"
            ><div>Submit</div></button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
    </form>
</div>
