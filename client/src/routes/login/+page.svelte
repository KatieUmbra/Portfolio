<script lang="ts">
    import type { ActionData } from "./$types";

    export let form: ActionData;
</script>

<div
    class="gird-cols-1 grid min-h-screen w-full items-center justify-center gap-2 bg-pink-200">
    <form action="/login" method="POST">
        <p class="m-3 text-xl font-bold">Login</p>
        <input
            type="text"
            class="max-w-ws input input-bordered m-3 w-full"
            name="username"
            placeholder="Username" />
        <input
            type="password"
            class="max-w-ws input input-bordered m-3 w-full"
            name="password"
            placeholder="Password" />
        <button type="submit" class="btn m-3">Submit</button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
    </form>
</div>
