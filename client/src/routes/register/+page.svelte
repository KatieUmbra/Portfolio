<script lang="ts">
    import type { ActionData } from "./$types";

    import { windowTitle } from "$lib/stores/global";

    windowTitle.set("Register");

    let { form }: { form: ActionData } = $props();
</script>

<form action="/register" method="POST">
    <div class="grid">
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
        <input
            type="password"
            class="txt-in95 m-3"
            name="verifyPassword"
            placeholder="Verify Password" />
        <button type="submit" class="btn95 m-3 max-w-min"
            ><div>Submit</div></button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
    </div>
</form>
