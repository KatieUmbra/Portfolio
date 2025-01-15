<script lang="ts">
    import type { ActionData } from "./$types";

    interface Props {
        form: ActionData;
    }

    let { form }: Props = $props();
</script>

<div class="max-h-95 bg-95 grid min-h-95 w-full items-center justify-center overflow-scroll">
    <form action="/login" method="POST" class="grid bg-white">
        <p class="m-3 text-xl font-bold">Login</p>
        <input
            type="text"
            class="txt-in95 m-3"
            name="username"
            placeholder="Username" />
        <input
            type="password"
            class="txt-in95 m-3"
            name="password"
            placeholder="Password" />
        <button type="submit" class="btn95 m-3">Submit</button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
    </form>
</div>
