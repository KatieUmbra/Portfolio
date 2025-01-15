<script lang="ts">
    import "../../app.css";
    import type { ActionData } from "./$types";

    interface Props {
        form: ActionData;
    }

    let { form }: Props = $props();
</script>

<div class="max-h-95 bg-95 grid min-h-95 w-full items-center justify-center overflow-scroll">
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
    </form>
</div>
