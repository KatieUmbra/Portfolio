<script lang="ts">
    import type { ActionData } from "./$types";

    export let form: ActionData;
</script>

<div class="grid gird-cols-1 gap-2 bg-pink-200 w-full min-h-screen items-center justify-center">
    <form action="/login" method="POST">
        <p class="m-3 text-xl font-bold">Login</p>
        <input type="text" class="input input-bordered w-full max-w-ws m-3" name="username" placeholder="Username"/>
        <input type="password" class="input input-bordered w-full max-w-ws m-3" name="password" placeholder="Password"/>
        <button type="submit" class="btn m-3">Submit</button>
    {#if form?.failure}
        <p class="m-3">{form?.message}</p>
    {/if}
    </form>
</div>
